mod bind_group;
mod bind_group_layout;
mod buffer;
mod command_encoder;
mod pipeline;
mod pipeline_layout;
mod render_device;
mod render_pass;
mod resource_macros;
mod sampler;
mod shader_module;
mod surface;
mod texture;
mod texture_view;

pub use bind_group::*;
pub use bind_group_layout::*;
pub use buffer::*;
pub use command_encoder::*;
pub use pipeline::*;
pub use pipeline_layout::*;
pub use render_device::*;
pub use render_pass::*;
pub use sampler::*;
pub use shader_module::*;
pub use surface::*;
pub use texture::*;
pub use texture_view::*;

pub use wgpu::{ShaderModuleDescriptor, ShaderSource};

use wgpu::{Adapter, BufferAddress, Instance, Queue, SurfaceTarget};

use std::sync::Arc;

#[derive(Clone)]
pub struct RenderQueue(Arc<Queue>);

impl RenderQueue {
    pub fn new(queue: Queue) -> Self {
        Self(Arc::new(queue))
    }

    pub fn write_buffer(&self, buffer: &GpuBuffer, offset: BufferAddress, data: &[u8]) {
        self.0.write_buffer(buffer.get_wgpu_buffer(), offset, data);
    }
}

pub struct RenderInstance(Arc<Instance>);

impl RenderInstance {
    pub fn new(instance: Instance) -> Self {
        Self(Arc::new(instance))
    }

    // SAFETY: The window handles in ExtractedWindows will always be valid objects to create surfaces on
    pub fn create_surface(&self, target: impl Into<SurfaceTarget<'static>>) -> GpuSurface {
        let surface = self
            .0
            .create_surface(target)
            .expect("Failed to create wgpu surface");

        GpuSurface::new(surface)
    }
}

#[derive(Clone)]
pub struct RenderAdapter(Arc<Adapter>);

impl RenderAdapter {
    pub fn new(value: Adapter) -> Self {
        RenderAdapter(Arc::new(value))
    }

    pub(crate) fn get_wgpu_adpter(&self) -> &Adapter {
        &self.0
    }
}
