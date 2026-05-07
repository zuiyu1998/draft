use draft_window::{SystemWindow, SystemWindowManager};

pub struct RenderServer {
    pub device: RenderDevice,
}

pub struct RenderDevice {
    pub device: wgpu::Device,
}

impl RenderDevice {
    pub fn new(device: wgpu::Device) -> Self {
        RenderDevice { device }
    }
}

impl RenderServer {
    pub async fn initialize(_window: SystemWindow) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        let instance = wgpu::Instance::new(instance_descriptor);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, _queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        RenderServer {
            device: RenderDevice::new(device),
        }
    }
}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_server,
            system_window_manager,
        }
    }
}
