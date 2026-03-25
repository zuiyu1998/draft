use std::collections::HashMap;

use draft_core::pool::Handle;
use draft_window::{RawHandleWrapper, SystemWindow};
use wgpu::{SurfaceTargetUnsafe, TextureFormat};

use crate::render_server::{RenderAdapter, RenderDevice, RenderInstance};

pub struct WindowSurface {
    surface: wgpu::Surface<'static>,
    pub surface_format: TextureFormat,
    surface_configuration: wgpu::SurfaceConfiguration,
}

impl WindowSurface {
    pub fn new(instance: &RenderInstance, adapter: &RenderAdapter, window: &SystemWindow) -> Self {
        let size = window.inner_size();
        let window = RawHandleWrapper::new(&window).unwrap();

        let surface_target = SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: Some(window.get_display_handle()),
            raw_window_handle: window.get_window_handle(),
        };
        // SAFETY: The window handles in ExtractedWindows will always be valid objects to create surfaces on
        let surface = unsafe {
            // NOTE: On some OSes this MUST be called from the main thread.
            // As of wgpu 0.15, only fallible if the given window is a HTML canvas and obtaining a WebGPU or WebGL2 context fails.
            instance
                .0
                .create_surface_unsafe(surface_target)
                .expect("Failed to create wgpu surface")
        };
        let cap = surface.get_capabilities(&adapter.0);
        let surface_format = cap.formats[0];

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        Self {
            surface,
            surface_format,
            surface_configuration,
        }
    }

    pub fn configure_surface(&mut self, device: &RenderDevice, window: &SystemWindow) {
        let size = window.inner_size();

        self.surface_configuration.height = size.height;
        self.surface_configuration.width = size.width;
        self.surface_configuration.view_formats = vec![self.surface_format.add_srgb_suffix()];

        self.surface
            .configure(&device.device, &self.surface_configuration);
    }

    pub fn get_current_texture(&self) -> wgpu::CurrentSurfaceTexture {
        self.surface.get_current_texture()
    }
}

#[derive(Default)]
pub struct WindowSurfaces {
    pub data: HashMap<Handle<SystemWindow>, WindowSurface>,
}
