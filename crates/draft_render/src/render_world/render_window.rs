use std::collections::HashMap;

use draft_graphics::{RenderDevice, RenderServer, Surface, SurfaceConfiguration, SurfaceTexture};
use draft_window::SystemWindow;
use fyrox_resource::core::pool::Handle;
use wgpu::TextureFormat;

pub struct RenderWindow {
    pub handle: Handle<SystemWindow>,
    pub physical_width: u32,
    pub physical_height: u32,
    pub surface_config: SurfaceConfiguration,
    pub surface: Surface<'static>,
    pub surface_format: TextureFormat,

    pub swap_chain_texture: Option<SurfaceTexture>,
}

impl RenderWindow {
    pub fn initialize(
        render_server: &RenderServer,
        handle: Handle<SystemWindow>,
        system_window: &SystemWindow,
    ) -> Self {
        let size = system_window.get_window().get_physical_size();
        let surface = render_server.create_surface(&system_window);

        let caps = surface.get_capabilities(&render_server.adapter);

        let formats = caps.formats;
        let format = *formats.first().expect("No supported formats for surface");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        Self {
            handle,
            physical_width: size.width,
            physical_height: size.height,
            surface_config,
            surface,
            surface_format: format,
            swap_chain_texture: None,
        }
    }

    pub fn configure(&self, device: &RenderDevice) {
        self.surface
            .configure(device.wgpu_device(), &self.surface_config);
    }
}

#[derive(Default)]
pub struct RenderWindowContainer {
    windows: HashMap<Handle<SystemWindow>, RenderWindow>,
}

impl RenderWindowContainer {
    pub fn get_or_create(
        &mut self,
        render_server: &RenderServer,
        handle: Handle<SystemWindow>,
        window: &SystemWindow,
    ) -> &mut RenderWindow {
        if !self.windows.contains_key(&handle) {
            let render_window = RenderWindow::initialize(render_server, handle, &window);
            render_window.configure(&render_server.device);
            self.windows.insert(handle.clone(), render_window);
        }

        self.windows.get_mut(&handle).unwrap()
    }
}
