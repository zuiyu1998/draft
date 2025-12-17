mod render_data_bundle;
mod window;

use draft_graphics::{
    gfx_base::{BufferDescriptor, GpuBuffer, RenderQueue},
    wgpu::BufferUsages,
};
pub use render_data_bundle::*;
pub use window::*;

use draft_geometry::{GeometryResource, GeometryVertexBufferLayouts};

use crate::{BufferAllocator, CachedRenderPipelineId, PipelineCache, error::FrameworkError};

pub struct Frame {
    pub windows: RenderWindows,
    pub render_data_bundle: RenderDataBundle,
}

fn create_vertext_buffer(
    geomertry: &GeometryResource,
    buffer_allocator: &mut BufferAllocator,
    render_queue: &RenderQueue,
) -> GpuBuffer {
    let geomertry = geomertry.data_ref();
    let vertex_buffer_size = geomertry.get_vertex_buffer_size();

    let desc = BufferDescriptor {
        label: None,
        size: vertex_buffer_size as u64,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };
    let vertext_buffer_handle = buffer_allocator.allocate(desc);
    let buffer = buffer_allocator.get_buffer(vertext_buffer_handle);
    let data = geomertry.create_packed_vertex_buffer_data();
    render_queue.write_buffer(&buffer, 0, &data);

    buffer
}

fn create_index_buffer(
    geomertry: &GeometryResource,
    buffer_allocator: &mut BufferAllocator,
    render_queue: &RenderQueue,
) -> Option<GpuBuffer> {
    let geomertry = geomertry.data_ref();
    let index = geomertry.get_index_buffer_bytes();

    if index.is_none() {
        return None;
    }

    let index = index.unwrap();

    let desc = BufferDescriptor {
        label: None,
        size: index.len() as u64,
        usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };
    let index_buffer_handle = buffer_allocator.allocate(desc);
    let buffer = buffer_allocator.get_buffer(index_buffer_handle);
    let data = geomertry.create_packed_vertex_buffer_data();
    render_queue.write_buffer(&buffer, 0, &data);

    Some(buffer)
}

pub struct BatchRenderMesh {
    pub vertex_buffer: GpuBuffer,
    pub index_buffer: Option<GpuBuffer>,
    pub id: CachedRenderPipelineId,
}

impl Frame {
    pub fn prepare(
        self,
        specialized_mesh_pipeline: &mut SpecializedMeshPipeline,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut GeometryVertexBufferLayouts,
        buffer_allocator: &mut BufferAllocator,
        render_queue: &RenderQueue,
    ) -> Result<RenderFrame, FrameworkError> {
        let mut batchs = vec![];

        for batch in self.render_data_bundle.mesh.values() {
            let vertex_buffer =
                create_vertext_buffer(&batch.geometry, buffer_allocator, render_queue);
            let index_buffer = create_index_buffer(&batch.geometry, buffer_allocator, render_queue);
            let id = specialized_mesh_pipeline.get(batch, pipeline_cache, layouts)?;

            batchs.push(BatchRenderMesh {
                vertex_buffer,
                id,
                index_buffer,
            });
        }

        Ok(RenderFrame {
            windows: self.windows,
        })
    }
}

pub struct RenderFrame {
    pub windows: RenderWindows,
}
