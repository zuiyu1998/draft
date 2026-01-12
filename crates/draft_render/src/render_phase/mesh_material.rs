use std::ops::Range;

use draft_graphics::{IndexFormat, frame_graph::TransientBindGroup, gfx_base::CachedPipelineId};
use draft_mesh::MeshResource;

use crate::{DrawError, RenderPhase, RenderPhaseContext, TrackedRenderPassBuilder};

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

    pub fn render(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
        batch_range: &Range<u32>,
    ) -> Result<(), DrawError> {
        let Some(vertex_buffer_slice) = context.mesh_allocator.mesh_vertex_slice(&self.key) else {
            return Ok(());
        };

        builder.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));

        match &self.indice_info {
            RenderIndiceInfo::Indexed {
                count,
                index_format,
            } => {
                let Some(index_buffer_slice) = context.mesh_allocator.mesh_index_slice(&self.key)
                else {
                    return Ok(());
                };

                builder.set_index_buffer(*index_format, index_buffer_slice.buffer.slice(..));

                builder.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    batch_range.clone(),
                );
            }
            RenderIndiceInfo::NonIndexed => {
                builder.draw(vertex_buffer_slice.range, batch_range.clone());
            }
        }

        Ok(())
    }
}

pub enum RenderIndiceInfo {
    Indexed {
        count: u32,
        index_format: IndexFormat,
    },
    NonIndexed,
}

pub struct RenderBindGroup {
    index: usize,
    bind_group: TransientBindGroup,
    offsets: Vec<u32>,
}

impl RenderBindGroup {
    pub fn render(&self, builder: &mut TrackedRenderPassBuilder) {
        builder.set_bind_group(self.index, &self.bind_group, &self.offsets);
    }
}

pub struct BatchRenderMeshMaterial {
    pub pipeline_id: CachedPipelineId,
    pub bind_groups: Vec<RenderBindGroup>,
    pub mesh_info: RenderMeshInfo,
    pub batch_range: Range<u32>,
}

impl RenderPhase for BatchRenderMeshMaterial {
    fn render(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> Result<(), DrawError> {
        let Some(pipeline) = context
            .pipeline_container
            .get_render_pipeline(self.pipeline_id)
        else {
            return Ok(());
        };
        builder.set_render_pipeline(pipeline);

        for bind_group in self.bind_groups.iter() {
            bind_group.render(builder);
        }

        self.mesh_info.render(builder, context, &self.batch_range)?;

        Ok(())
    }
}
