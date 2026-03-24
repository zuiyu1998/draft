mod render_device;

pub use render_device::*;

use std::sync::Arc;

pub struct RenderQueue(pub Arc<wgpu::Queue>);

impl RenderQueue {
    pub fn new(queue: wgpu::Queue) -> Self {
        RenderQueue(Arc::new(queue))
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
}

#[derive(Default, Clone, Debug)]
pub struct RenderServerSetting {}
