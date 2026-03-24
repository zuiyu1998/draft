mod render_device;

pub use render_device::*;

use std::sync::Arc;

pub struct RenderAdapter(pub Arc<wgpu::Adapter>);

impl RenderAdapter {
    pub fn new(adapter: wgpu::Adapter) -> Self {
        RenderAdapter(Arc::new(adapter))
    }
}

pub struct RenderInstance(pub Arc<wgpu::Instance>);

impl RenderInstance {
    pub fn new(instance: wgpu::Instance) -> Self {
        RenderInstance(Arc::new(instance))
    }
}

pub struct RenderQueue(pub Arc<wgpu::Queue>);

impl RenderQueue {
    pub fn new(queue: wgpu::Queue) -> Self {
        RenderQueue(Arc::new(queue))
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}

#[derive(Default, Clone, Debug)]
pub struct RenderServerSetting {}
