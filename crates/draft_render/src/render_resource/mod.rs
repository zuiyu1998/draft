mod window_surface;
mod window_surface_texture;

use std::mem::take;

pub use window_surface::*;
pub use window_surface_texture::*;

#[derive(Default)]
pub struct RenderWorld {
    pub(crate) window_surface_textures: WindowSurfaceTextures,
}

impl RenderWorld {
    pub fn prepare_window_surface_textures(&mut self, window_surfaces: &WindowSurfaces) {
        for (handle, window_surface) in window_surfaces.data.iter() {
            self.window_surface_textures.insert(handle, window_surface);
        }
    }

    pub fn clear_window_surface_textures(&mut self) {
        let window_surface_textures = take(&mut self.window_surface_textures);

        for window_surface in window_surface_textures.data.into_values() {
            window_surface.present();
        }
    }
}
