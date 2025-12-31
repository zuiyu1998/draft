use draft_graphics::{IndexFormat, gfx_base::CachedPipelineId};
use draft_material::{MaterialClass, MaterialResource};

use draft_mesh::{MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts};
use fxhash::FxHashMap;
use std::{
    collections::hash_map::Entry,
    ops::{Deref, DerefMut, Range},
};

use crate::{
    CachedRenderPipelineId, FrameworkError, MeshMaterialPipeline, PipelineCache, RenderPhase,
    RenderPhaseContext, TrackedRenderPassBuilder,
};

#[derive(Default)]
pub struct BatchRenderMeshMaterialContainer(
    FxHashMap<BatchMeshMaterialKey, Vec<BatchRenderMeshMaterial>>,
);

impl Deref for BatchRenderMeshMaterialContainer {
    type Target = FxHashMap<BatchMeshMaterialKey, Vec<BatchRenderMeshMaterial>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BatchRenderMeshMaterialContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct BatchRenderMeshMaterial {
    pub pipeline_id: CachedPipelineId,
    pub material: MaterialResource,
    pub mesh_info: RenderMeshInfo,
    pub batch_range: Range<u32>,
}

pub enum RenderIndiceInfo {
    Indexed {
        count: u32,
        index_format: IndexFormat,
    },
    NonIndexed,
}

pub struct RenderMeshInfo {
    pub key: u64,
    pub indice_info: RenderIndiceInfo,
}

impl RenderMeshInfo {
    pub fn from_mesh(mesh: &MeshResource) -> Self {
        let key = mesh.key();
        let mesh = mesh.data_ref();

        let indice_info = match mesh.indices() {
            None => RenderIndiceInfo::NonIndexed,
            Some(indices) => RenderIndiceInfo::Indexed {
                count: indices.len() as u32,
                index_format: indices.index_format(),
            },
        };

        RenderMeshInfo { key, indice_info }
    }
}

impl RenderPhase for BatchRenderMeshMaterial {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, context: &RenderPhaseContext) {
        let Some(pipeline) = context
            .pipeline_container
            .get_render_pipeline(self.pipeline_id)
        else {
            return;
        };

        let Some(vertex_buffer_slice) = context
            .mesh_allocator
            .mesh_vertex_slice(&self.mesh_info.key)
        else {
            return;
        };

        builder.set_render_pipeline(pipeline);

        builder.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));

        match &self.mesh_info.indice_info {
            RenderIndiceInfo::Indexed {
                count,
                index_format,
            } => {
                let Some(index_buffer_slice) =
                    context.mesh_allocator.mesh_index_slice(&self.mesh_info.key)
                else {
                    return;
                };

                builder.set_index_buffer(*index_format, index_buffer_slice.buffer.slice(..));

                builder.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    self.batch_range.clone(),
                );
            }
            RenderIndiceInfo::NonIndexed => {
                builder.draw(vertex_buffer_slice.range, self.batch_range.clone());
            }
        }
    }
}

pub struct MeshInstanceData {}

pub struct BatchMeshMaterial {
    pub mesh: MeshResource,
    pub material: MaterialResource,
    pub instance: MeshInstanceData,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BatchMeshMaterialKey {
    pub mesh_layout: MeshVertexBufferLayoutRef,
    pub material_class: MaterialClass,
    pub pipeline_id: CachedRenderPipelineId,
}

impl BatchMeshMaterialKey {
    pub fn new(
        mesh: &MeshResource,
        material: &MaterialResource,
        layouts: &mut MeshVertexBufferLayouts,
        pipeline_id: CachedRenderPipelineId,
    ) -> BatchMeshMaterialKey {
        let mesh = mesh.data_ref();
        let mesh_layout = mesh.get_mesh_vertex_buffer_layout(layouts);

        let material = material.data_ref();
        let material_class = material.get_class();

        BatchMeshMaterialKey {
            mesh_layout,
            material_class,
            pipeline_id,
        }
    }
}

#[derive(Default)]
pub struct BatchMeshMaterialContainer(FxHashMap<BatchMeshMaterialKey, Vec<BatchMeshMaterial>>);

impl Deref for BatchMeshMaterialContainer {
    type Target = FxHashMap<BatchMeshMaterialKey, Vec<BatchMeshMaterial>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BatchMeshMaterialContainer {
    pub fn push(
        &mut self,
        mesh: MeshResource,
        material: MaterialResource,
        instance: MeshInstanceData,
        layouts: &mut MeshVertexBufferLayouts,
        mesh_material_pipeline: &mut MeshMaterialPipeline,
        pipeline_cache: &mut PipelineCache,
    ) -> Result<(), FrameworkError> {
        let pipeline_id = mesh_material_pipeline.get(&mesh, &material, pipeline_cache, layouts)?;
        let key = BatchMeshMaterialKey::new(&mesh, &material, layouts, pipeline_id);

        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(BatchMeshMaterial {
                    mesh,
                    material,
                    instance,
                });
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![BatchMeshMaterial {
                    mesh,
                    material,
                    instance,
                }]);
            }
        }

        Ok(())
    }
}
