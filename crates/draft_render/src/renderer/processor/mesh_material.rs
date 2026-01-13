use std::ops::Deref;

use draft_graphics::{BufferUsages, gfx_base::BindGroupLayout};
use draft_material::{Material, MaterialResource};
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use fxhash::FxHashMap;

use crate::{
    BatchMeshMaterialKey, BatchRenderMeshMaterial, BufferAllocator, BufferVec,
    MaterialEffectInstance, MeshMaterialInstanceData, MeshMaterialPipeline, PipelineCache,
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
}

pub enum MaterialResourceHandle {
    Buffer { key: String },
}

pub struct MaterialBindGroupEntryHandle {
    pub binding: u32,
    pub resource: MaterialResourceHandle,
}

pub struct MaterialBindLayoutHandle {
    pub label: String,
    pub layout: BindGroupLayout,
    pub entries: Vec<MaterialBindGroupEntryHandle>,
}

pub struct MaterialBindGroupHandle {
    pub name: String,
    pub layouts: Vec<MaterialBindLayoutHandle>,
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
    pub bind_groups: Vec<MaterialBindGroupHandle>,
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
            let mut layouts = vec![];

            for bind_group_layout in bind_group.bind_group_layouts.iter() {
                let mut entries = vec![];

                for (index, resource_binding) in
                    bind_group_layout.resource_bindings.iter().enumerate()
                {
                    let material_extractor =
                        material_extractor_container.get_material_extractor(resource_binding);

                    let key = format!(
                        "{}-{}-{}",
                        bind_group.name, bind_group_layout.name, resource_binding
                    );

                    let mut context = MaterialExtractorContext {
                        resource_binding,
                        material,
                        instance_data,
                        buffer_allocator: &mut self.buffer_allocator,
                        key: &key,
                    };

                    let handle = material_extractor.extra(&mut context);

                    entries.push(MaterialBindGroupEntryHandle {
                        binding: index as u32,
                        resource: handle,
                    });
                }

                layouts.push(MaterialBindLayoutHandle {
                    label: bind_group_layout.name.to_string(),
                    layout: bind_group_layout.bind_group_layout.deref().clone(),
                    entries,
                });
            }

            groups.push(MaterialBindGroupHandle {
                name: bind_group.name.clone(),
                layouts,
            });
        }

        groups
    }

    pub fn push(&mut self, bind_groups: Vec<MaterialBindGroupHandle>) {
        self.count += 1;
        self.data.push(BatchMeshMaterialData { bind_groups });
    }

    pub fn allallocateow(
        self,
        _buffer_allocator: &mut BufferAllocator,
    ) -> Vec<BatchRenderMeshMaterial> {
        todo!()
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

        let key = BatchMeshMaterialKey::new(mesh, material, layouts, pipeline_id);

        let material = material.data_ref();

        let batch = self.container.get_or_create(&key);

        batch.extra_material(
            material_effect_instance,
            &material,
            &self.material_extractor_container,
            instance_data,
        );
    }
}
