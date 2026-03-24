use std::sync::Arc;

use draft_core::parking_lot::Mutex;
use draft_render::{
    FrameworkError,
    render_server::{RenderDevice, RenderQueue, RenderServer},
    wgpu,
};
use draft_window::{SystemWindow, Window};
use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

use super::{RenderServerConstructor, RenderServerSetting};

#[derive(Default, Clone)]
pub struct FutureRenderServer(Arc<Mutex<Option<RenderServer>>>);

impl RenderServerConstructor for ActiveEventLoop {
    fn construct(
        &self,
        _setting: &RenderServerSetting,
        _window: Window,
    ) -> Result<(RenderServer, SystemWindow), FrameworkError> {
        let window = self
            .create_window(WindowAttributes::default())
            .expect("create window faild.");

        let window = SystemWindow::new(window);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(self.owned_display_handle()),
        ));

        let future_render_server = FutureRenderServer::default();

        pollster::block_on(async {
            initialize_render_server(future_render_server.clone(), instance).await;
        });
        let mut guard = future_render_server.0.lock();
        let render_server = guard.take().unwrap();

        Ok((render_server, window))
    }
}

async fn initialize_render_server(
    future_render_server: FutureRenderServer,
    instance: wgpu::Instance,
) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .unwrap();

    let render_server = RenderServer {
        device: RenderDevice::new(device),
        queue: RenderQueue::new(queue),
    };

    let mut guard = future_render_server.0.lock();
    *guard = Some(render_server);
}
