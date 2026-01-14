use std::{mem::take, num::NonZero, ops::Deref};

use draft_graphics::{
    BufferUsages,
    gfx_base::{BindGroupLayout, RenderQueue},
};
use draft_material::{Material, MaterialResource};
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use fxhash::FxHashMap;

use crate::{
    BatchMeshMaterialKey, BatchRenderMeshMaterial, BatchRenderMeshMaterialContainer,
    BufferAllocator, BufferHandle, BufferVec, CachedRenderPipelineId, MaterialEffectInstance,
    MeshMaterialInstanceData, MeshMaterialPipeline, PipelineCache, RenderBindGroup,
    RenderBufferHandle, RenderMeshInfo, RenderTransientBindGroup, RenderTransientBindGroupEntry,
    RenderTransientBindGroupResource,
};

#[derive(Default)]
pub struct BatchMeshMateriaBufferAllocator {
    buffers: FxHashMap<String, BufferVec>,
}

impl BatchMeshMateriaBufferAllocator {
    pub fn get_or_create(&mut self, key: &str, buffer_usage: BufferUsages) -> &mut BufferVec {
        if let None = self.buffers.get(key) {
            self.buffers
                .insert(key.to_string(), BufferVec::new(buffer_usage, key));
        }

        self.buffers.get_mut(key).unwrap()
    }
}

pub struct MaterialExtractorContext<'a> {
    pub resource_binding: &'a str,
    pub material: &'a Material,
    pub instance_data: &'a MeshMaterialInstanceData,
    pub buffer_allocator: &'a mut BatchMeshMateriaBufferAllocator,
    pub key: &'a str,
    pub offsets: &'a mut Vec<u32>,
}

pub enum MaterialResourceHandle {
    Buffer {
        key: String,
        size: Option<NonZero<u64>>,
        offset: u64,
    },
}

impl MaterialResourceHandle {
    pub fn create_render_transient_bind_group_resource(
        &self,
        buffer_allocator: &FxHashMap<String, BufferHandle>,
    ) -> RenderTransientBindGroupResource {
        match self {
            MaterialResourceHandle::Buffer { key, size, offset } => {
                RenderTransientBindGroupResource::Buffer(RenderBufferHandle {
                    handle: buffer_allocator.get(key).unwrap().clone(),
                    size: *size,
                    offset: *offset,
                    key: key.clone(),
                })
            }
        }
    }
}

pub struct MaterialBindGroupEntryHandle {
    pub binding: u32,
    pub resource: MaterialResourceHandle,
}

impl MaterialBindGroupEntryHandle {
    pub fn create_render_transient_bind_group_entry(
        &self,
        buffer_allocator: &FxHashMap<String, BufferHandle>,
    ) -> RenderTransientBindGroupEntry {
        RenderTransientBindGroupEntry {
            binding: self.binding,
            resource: self
                .resource
                .create_render_transient_bind_group_resource(buffer_allocator),
        }
    }
}

pub struct MaterialBindGroupHandle {
    pub name: String,
    pub layout: BindGroupLayout,
    pub entries: Vec<MaterialBindGroupEntryHandle>,
    pub offsets: Vec<u32>,
}

impl MaterialBindGroupHandle {
    pub fn create_render_transient_bind_group(
        &self,
        buffer_allocator: &FxHashMap<String, BufferHandle>,
    ) -> RenderTransientBindGroup {
        let entries = self
            .entries
            .iter()
            .map(|entry| entry.create_render_transient_bind_group_entry(buffer_allocator))
            .collect();

        RenderTransientBindGroup {
            label: Some(self.name.clone()),
            layout: self.layout.clone(),
            entries,
        }
    }
}

pub trait MaterialExtractor: 'static {
    fn extra(&self, context: &mut MaterialExtractorContext) -> MaterialResourceHandle;

    fn clone_boxed(&self) -> Box<dyn MaterialExtractor>;
}

#[derive(Clone)]
pub struct MeshMaterialExtractor;

impl MaterialExtractor for MeshMaterialExtractor {
    fn extra(&self, context: &mut MaterialExtractorContext) -> MaterialResourceHandle {
        let buffer = context
            .buffer_allocator
            .get_or_create(context.key, BufferUsages::STORAGE);

        buffer.push(&context.instance_data.data);

        MaterialResourceHandle::Buffer {
            key: context.key.to_string(),
            size: None,
            offset: 0,
        }
    }

    fn clone_boxed(&self) -> Box<dyn MaterialExtractor> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct DefalutMaterialExtractor;

impl MaterialExtractor for DefalutMaterialExtractor {
    fn clone_boxed(&self) -> Box<dyn MaterialExtractor> {
        Box::new(self.clone())
    }

    fn extra(&self, context: &mut MaterialExtractorContext) -> MaterialResourceHandle {
        MaterialResourceHandle::Buffer {
            key: context.key.to_string(),
            size: None,
            offset: 0,
        }
    }
}

pub struct MaterialExtractorContainer(FxHashMap<String, Box<dyn MaterialExtractor>>);

impl Default for MaterialExtractorContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialExtractorContainer {
    pub fn new() -> Self {
        let mut empty = MaterialExtractorContainer::empty();

        empty
            .0
            .insert("draft_mesh_2d".to_string(), Box::new(MeshMaterialExtractor));

        empty
    }

    pub fn empty() -> Self {
        MaterialExtractorContainer(Default::default())
    }

    pub fn get_material_extractor(&self, resource_binding: &str) -> Box<dyn MaterialExtractor> {
        self.0
            .get(resource_binding)
            .map(|v| v.clone_boxed())
            .unwrap_or_else(|| Box::new(DefalutMaterialExtractor))
    }
}

pub struct BatchMeshMaterialData {
    pub pipeline_id: CachedRenderPipelineId,
    pub bind_groups: Vec<MaterialBindGroupHandle>,
    pub mesh_info: RenderMeshInfo,
}

#[derive(Default)]
pub struct BatchMeshMaterial {
    count: u32,
    buffer_allocator: BatchMeshMateriaBufferAllocator,
    data: Vec<BatchMeshMaterialData>,
}

impl BatchMeshMaterial {
    pub fn extra_material(
        &mut self,
        material_effect_instance: &MaterialEffectInstance,
        material: &Material,
        material_extractor_container: &MaterialExtractorContainer,
        instance_data: &MeshMaterialInstanceData,
    ) -> Vec<MaterialBindGroupHandle> {
        let mut groups = vec![];

        for bind_group in material_effect_instance.bind_groups.iter() {
            let mut entries = vec![];

            let mut offsets = vec![];

            for (index, resource_binding) in bind_group.resource_bindings.iter().enumerate() {
                let material_extractor =
                    material_extractor_container.get_material_extractor(resource_binding);

                let key = format!("{}-{}", bind_group.name, resource_binding);

                let mut context = MaterialExtractorContext {
                    resource_binding,
                    material,
                    instance_data,
                    buffer_allocator: &mut self.buffer_allocator,
                    key: &key,
                    offsets: &mut offsets,
                };

                let handle = material_extractor.extra(&mut context);

                entries.push(MaterialBindGroupEntryHandle {
                    binding: index as u32,
                    resource: handle,
                });
            }

            groups.push(MaterialBindGroupHandle {
                name: bind_group.name.clone(),
                layout: bind_group.bind_group_layout.deref().clone(),
                entries,
                offsets,
            });
        }

        groups
    }

    pub fn push(&mut self, data: BatchMeshMaterialData) {
        self.count += 1;
        self.data.push(data);
    }

    pub fn allocate(
        self,
        buffer_allocator: &mut BufferAllocator,
        render_queue: &RenderQueue,
    ) -> Vec<BatchRenderMeshMaterial> {
        let buffer_allocator = self
            .buffer_allocator
            .buffers
            .into_iter()
            .map(|(key, mut buffer_vec)| {
                let handle = buffer_vec.write_buffer(buffer_allocator, render_queue);
                (key, handle)
            })
            .collect::<FxHashMap<String, BufferHandle>>();

        let mut batchs = vec![];
        for data in self.data.into_iter() {
            let mut bind_groups = vec![];
            for (index, handle) in data.bind_groups.into_iter().enumerate() {
                let bind_group = handle.create_render_transient_bind_group(&buffer_allocator);
                bind_groups.push(RenderBindGroup {
                    index,
                    bind_group,
                    offsets: handle.offsets,
                });
            }

            batchs.push(BatchRenderMeshMaterial {
                pipeline_id: data.pipeline_id.id(),
                bind_groups,
                mesh_info: data.mesh_info,
                batch_range: 0..self.count,
            });
        }
        batchs
    }
}

#[derive(Default)]
pub struct BatchMeshMaterialContainer(FxHashMap<BatchMeshMaterialKey, BatchMeshMaterial>);

impl BatchMeshMaterialContainer {
    pub fn get_or_create(&mut self, key: &BatchMeshMaterialKey) -> &mut BatchMeshMaterial {
        if let None = self.0.get(key) {
            self.0.insert(key.clone(), BatchMeshMaterial::default());
        }

        self.0.get_mut(key).unwrap()
    }
}

pub struct MeshMaterialProcessor {
    mesh_material_pipeline: MeshMaterialPipeline,
    container: BatchMeshMaterialContainer,
    material_extractor_container: MaterialExtractorContainer,
}

impl MeshMaterialProcessor {
    pub fn new() -> Self {
        MeshMaterialProcessor {
            mesh_material_pipeline: Default::default(),
            container: Default::default(),
            material_extractor_container: Default::default(),
        }
    }

    pub fn process(
        &mut self,
        mesh: &MeshResource,
        material: &MaterialResource,
        instance_data: &MeshMaterialInstanceData,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut MeshVertexBufferLayouts,
        material_effect_instance: &MaterialEffectInstance,
    ) {
        let pipeline_id = self
            .mesh_material_pipeline
            .get(mesh, material, pipeline_cache, layouts);


        if pipeline_cache.get_pipeline(pipeline_id.id()).is_none() {
            return;
        }

        let key = BatchMeshMaterialKey::new(mesh, material, layouts, pipeline_id);

        let material = material.data_ref();

        let batch = self.container.get_or_create(&key);

        let bind_groups = batch.extra_material(
            material_effect_instance,
            &material,
            &self.material_extractor_container,
            instance_data,
        );

        batch.push(BatchMeshMaterialData {
            bind_groups,
            pipeline_id,
            mesh_info: RenderMeshInfo::from_mesh(mesh),
        });

    }

    pub fn update_cache(
        &mut self,
        buffer_allocator: &mut BufferAllocator,
        render_queue: &RenderQueue,
    ) -> BatchRenderMeshMaterialContainer {
        let container = take(&mut self.container);

        let data = container
            .0
            .into_iter()
            .map(|(key, batch_mesh_material)| {
                let batchs = batch_mesh_material.allocate(buffer_allocator, render_queue);
                (key, batchs)
            })
            .collect::<FxHashMap<BatchMeshMaterialKey, Vec<BatchRenderMeshMaterial>>>();

        BatchRenderMeshMaterialContainer::new(data)
    }
}
