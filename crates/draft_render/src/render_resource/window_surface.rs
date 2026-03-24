use std::collections::HashMap;

use draft_window::{PhysicalSize, RawHandleWrapper, SystemWindow};
use wgpu::{SurfaceTargetUnsafe, TextureFormat, naga::Handle};

use crate::render_server::{RenderAdapter, RenderDevice, RenderInstance};

pub struct WindowSurface {
    surface: wgpu::Surface<'static>,
    surface_format: TextureFormat,
    size: PhysicalSize,
}

impl WindowSurface {
    pub fn new(instance: &RenderInstance, adapter: &RenderAdapter, window: SystemWindow) -> Self {
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

        Self {
            surface,
            surface_format,
            size,
        }
    }

    pub fn configure_surface(&self, device: &RenderDevice) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&device.device, &surface_config);
    }
}

#[derive(Default)]
pub struct WindowSurfaces {
    pub data: HashMap<Handle<SystemWindow>, WindowSurface>,
}
