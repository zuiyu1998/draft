mod device;

use draft_window::SystemWindow;

pub use device::*;

#[derive(Clone)]
pub struct RenderServer {
    pub device: RenderDevice,
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
