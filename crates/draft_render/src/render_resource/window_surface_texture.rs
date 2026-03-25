use std::collections::HashMap;

use draft_core::pool::Handle;
use draft_window::SystemWindow;
use wgpu::SurfaceTexture;

use crate::render_resource::WindowSurface;

pub struct WindowSurfaceTexture {
    pub(crate) surface: SurfaceTexture,
}

impl WindowSurfaceTexture {
    pub fn present(self) {
        self.surface.present();
    }
}

#[derive(Default)]
pub struct WindowSurfaceTextures {
    pub(crate) data: HashMap<Handle<SystemWindow>, WindowSurfaceTexture>,
}

impl WindowSurfaceTextures {
    pub fn get_window_surface_texture(
        &self,
        handle: &Handle<SystemWindow>,
    ) -> Option<&WindowSurfaceTexture> {
        self.data.get(handle)
    }

    pub fn insert(&mut self, handle: &Handle<SystemWindow>, window_surface: &WindowSurface) {
        match window_surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => self
                .data
                .insert(*handle, WindowSurfaceTexture { surface: texture }),
            _ => {
                panic!("get_current_texture faild.")
            }
        };
    }
}
