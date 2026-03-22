mod render_device;

pub use render_device::*;

use crate::{FrameworkError, GraphicsContextParams};
use draft_window::{SystemWindow, Window};
use std::sync::Arc;

pub trait RenderServerConstructor {
    fn construct(
        &self,
        params: &GraphicsContextParams,
        window: Window,
    ) -> Result<(RenderServer, SystemWindow), FrameworkError>;
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
}

impl RenderServer {
    pub fn new<T: RenderServerConstructor>(
        params: &GraphicsContextParams,
        window: Window,
        constructor: &T,
    ) -> Result<(Self, SystemWindow), FrameworkError> {
        constructor.construct(params, window)
    }
}
