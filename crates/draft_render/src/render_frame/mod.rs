mod mesh_material;
mod window;

use std::ops::Range;

use draft_mesh::MeshResource;
pub use mesh_material::*;
pub use window::*;

use draft_graphics::{IndexFormat, gfx_base::CachedPipelineId};
use draft_material::MaterialResource;

use crate::{RenderPhase, RenderPhaseContext, TrackedRenderPassBuilder};

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
                count: indices.count() as u32,
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

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub batchs: Vec<BatchRenderMeshMaterial>,
}
