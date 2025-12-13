use draft_graphics::frame_graph::gfx_base::{RenderDevice, RenderQueue};
use draft_window::RawHandleWrapper;
use fyrox_core::futures::executor::block_on;
use wgpu::{Instance, InstanceDescriptor, RequestAdapterOptions, wgt::DeviceDescriptor};

pub struct RenderServer {
    pub decice: RenderDevice,
    pub queue: RenderQueue,
}

pub fn initialize_render_server(primary_window: RawHandleWrapper) -> RenderServer {
    block_on(async { initialize_render_server_async(primary_window).await })
}

pub async fn initialize_render_server_async(primary_window: RawHandleWrapper) -> RenderServer {
    let instance = Instance::new(&InstanceDescriptor::default());

    // SAFETY: Plugins should be set up on the main thread.
    let handle = unsafe { primary_window.get_handle() };

    let surface = instance
        .create_surface(handle)
        .expect("Failed to create wgpu surface");

    let request_adapter_options = RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    };

    let adapter = instance
        .request_adapter(&request_adapter_options)
        .await
        .ok()
        .expect("Failed to create adapter");

    let (device, queue) = adapter
        .request_device(&DeviceDescriptor::default())
        .await
        .unwrap();

    RenderServer {
        decice: RenderDevice::new(device),
        queue: RenderQueue::new(queue),
    }
}
