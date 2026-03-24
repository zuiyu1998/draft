use draft_render::{FrameworkError, render_server::RenderServer, wgpu};
use draft_window::{SystemWindow, Window};
use winit::{event_loop::ActiveEventLoop, window::WindowAttributes};

use super::{RenderServerConstructor, RenderServerSetting};

impl RenderServerConstructor for ActiveEventLoop {
    fn construct(
        &self,
        _setting: &RenderServerSetting,
        _window: Window,
    ) -> Result<(RenderServer, SystemWindow), FrameworkError> {
        let _window = self
            .create_window(WindowAttributes::default())
            .expect("create window faild.");

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(self.owned_display_handle()),
        ));

        pollster::block_on(async {
            initialize_render_server(instance).await;
        });

        todo!()
    }
}

async fn initialize_render_server(instance: wgpu::Instance) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (_device, _queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .unwrap();
}
