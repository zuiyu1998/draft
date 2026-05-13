mod device;

use std::{ops::Deref, sync::Arc};

use draft_window::SystemWindow;
use wgpu::{Surface, SurfaceTargetUnsafe};

pub use device::*;

#[derive(Clone)]
pub struct RenderQueue(Arc<wgpu::Queue>);

impl Deref for RenderQueue {
    type Target = wgpu::Queue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderQueue {
    pub fn new(queue: wgpu::Queue) -> Self {
        Self(Arc::new(queue))
    }
}

#[derive(Clone)]
pub struct RenderInstance(Arc<wgpu::Instance>);

impl RenderInstance {
    pub fn new(instance: wgpu::Instance) -> Self {
        Self(Arc::new(instance))
    }
}

#[derive(Clone)]
pub struct RenderAdapter(Arc<wgpu::Adapter>);

impl Deref for RenderAdapter {
    type Target = wgpu::Adapter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderAdapter {
    pub fn new(adapter: wgpu::Adapter) -> Self {
        Self(Arc::new(adapter))
    }
}

#[derive(Clone)]
pub struct RenderServer {
    pub device: RenderDevice,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
    pub queue: RenderQueue,
}

impl RenderServer {
    pub fn create_surface(&self, window: &SystemWindow) -> Surface<'static> {
        let surface_target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(window.get_window().get_raw_display_handle()),
            raw_window_handle: window.get_window().get_raw_window_handle(),
        };

        let surface = unsafe {
            // NOTE: On some OSes this MUST be called from the main thread.
            // As of wgpu 0.15, only fallible if the given window is a HTML canvas and obtaining a WebGPU or WebGL2 context fails.
            self.instance
                .0
                .create_surface_unsafe(surface_target)
                .expect("Failed to create wgpu surface")
        };

        surface
    }

    pub async fn initialize(_window: SystemWindow) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        let instance = wgpu::Instance::new(instance_descriptor);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        RenderServer {
            device: RenderDevice::new(device),
            instance: RenderInstance::new(instance),
            adapter: RenderAdapter::new(adapter),
            queue: RenderQueue::new(queue)
        }
    }
}
