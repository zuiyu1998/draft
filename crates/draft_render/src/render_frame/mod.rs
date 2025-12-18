mod render_data_bundle;
mod window;

use draft_graphics::{
    gfx_base::{Buffer, BufferDescriptor, RenderQueue},
    wgpu::BufferUsages,
};
pub use render_data_bundle::*;
pub use window::*;

use draft_geometry::{GeometryResource, GeometryVertexBufferLayouts};

use crate::{
    BufferAllocator, BufferMeta, CachedRenderPipelineId, PipelineCache, error::FrameworkError,
};

pub struct Frame {
    pub windows: RenderWindows,
    pub render_data_bundle: RenderDataBundle,
}

fn create_vertext_buffer(
    geomertry: &GeometryResource,
    buffer_allocator: &mut BufferAllocator,
    render_queue: &RenderQueue,
) -> Buffer {
    let geomertry = geomertry.data_ref();
    let vertex_buffer_size = geomertry.get_vertex_buffer_size();

    let desc = BufferDescriptor {
        label: None,
        size: vertex_buffer_size as u64,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };

    let vertext_buffer_handle = buffer_allocator.allocate(desc.clone());
    let buffer = buffer_allocator.get_buffer(vertext_buffer_handle);
    let data = geomertry.create_packed_vertex_buffer_data();
    render_queue.write_buffer(&buffer, 0, &data);

    Buffer {
        value: buffer,
        desc,
    }
}

fn create_index_buffer(
    geomertry: &GeometryResource,
    buffer_allocator: &mut BufferAllocator,
    render_queue: &RenderQueue,
) -> Option<Buffer> {
    let geomertry = geomertry.data_ref();
    let data = geomertry.get_index_buffer_bytes();

    if data.is_none() {
        return None;
    }

    let data = data.unwrap();

    let desc = BufferDescriptor {
        label: None,
        size: data.len() as u64,
        usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    };
    let index_buffer_handle = buffer_allocator.allocate(desc.clone());
    let buffer = buffer_allocator.get_buffer(index_buffer_handle);
    render_queue.write_buffer(&buffer, 0, &data);

    Some(Buffer {
        value: buffer,
        desc,
    })
}

pub struct BatchRenderMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
    pub id: CachedRenderPipelineId,
}

impl BatchRenderMesh {
    pub fn get_vertex_buffer_meta(&self) -> BufferMeta {
        BufferMeta {
            key: "vertex".to_string(),
            value: self.vertex_buffer.clone(),
        }
    }

    pub fn get_index_buffer_meta(&self) -> Option<BufferMeta> {
        if self.index_buffer.is_none() {
            return None;
        }

        Some(BufferMeta {
            key: "index".to_string(),
            value: self.index_buffer.as_ref().unwrap().clone(),
        })
    }
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
            batchs,
        })
    }
}

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub batchs: Vec<BatchRenderMesh>,
}
