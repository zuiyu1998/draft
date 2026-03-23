
#[cfg(feature = "winit")]
mod winit;

mod render_device;

pub use render_device::*;

use crate::FrameworkError;
use draft_window::{SystemWindow, Window};
use std::sync::Arc;

pub trait RenderServerConstructor {
    fn construct(
        &self,
        setting: &RenderServerSetting,
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
        setting: &RenderServerSetting,
        window: Window,
        constructor: &T,
    ) -> Result<(Self, SystemWindow), FrameworkError> {
        constructor.construct(setting, window)
    }
}

#[derive(Default, Clone, Debug)]
pub struct RenderServerSetting {}
