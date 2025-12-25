mod render_data_bundle;
mod window;

use draft_graphics::gfx_base::RenderQueue;
pub use render_data_bundle::*;
pub use window::*;

use draft_mesh::MeshVertexBufferLayouts;

use crate::{BufferAllocator, PipelineCache, SpecializedMeshPipeline, error::FrameworkError};

pub struct Frame {
    pub windows: RenderWindows,
    pub render_data_bundle: RenderDataBundle,
}

impl Frame {
    pub fn prepare(
        self,
        _specialized_mesh_pipeline: &mut SpecializedMeshPipeline,
        _pipeline_cache: &mut PipelineCache,
        _layouts: &mut MeshVertexBufferLayouts,
        _buffer_allocator: &mut BufferAllocator,
        _render_queue: &RenderQueue,
    ) -> Result<RenderFrame, FrameworkError> {
        let batchs = vec![];

        // for batch in self.render_data_bundle.mesh.values() {
        //     let vertex_buffer =
        //         create_vertext_buffer(&batch.Mesh, buffer_allocator, render_queue);
        //     let index_buffer = create_index_buffer(&batch.Mesh, buffer_allocator, render_queue);
        //     let id = specialized_mesh_pipeline.get(batch, pipeline_cache, layouts)?;

        //     batchs.push(BatchRenderMesh {
        //         vertex_buffer,
        //         id,
        //         index_buffer,
        //     });
        // }

        Ok(RenderFrame {
            windows: self.windows,
            batchs,
        })
    }
}

pub struct BatchRenderMesh {}

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub batchs: Vec<BatchRenderMesh>,
}
