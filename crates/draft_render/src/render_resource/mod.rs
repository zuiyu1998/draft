mod mesh_cache;
mod temporary_cache;
mod texture_cache;
mod window_surface;
mod window_surface_texture;

use draft_mesh::MeshResource;
pub use mesh_cache::*;
pub use temporary_cache::*;
pub use texture_cache::*;
pub use window_surface::*;
pub use window_surface_texture::*;

use crate::{FrameworkError, render_server::RenderDevice};
use draft_image::ImageResource;
use fyrox_resource::manager::ResourceManager;
use std::mem::take;

#[derive(Default)]
pub struct RenderWorld {
    pub(crate) window_surface_textures: WindowSurfaceTextures,
    pub(crate) texture_cache: TextureCache,
    pub(crate) mesh_cache: MeshCache,
}

impl RenderWorld {
    pub fn upload_mesh(&mut self, mesh: &MeshResource) {
        self.mesh_cache.get(mesh);
    }

    pub fn update_mesh_cache(&mut self, dt: f32) {
        self.mesh_cache.update(dt);
    }

    pub fn update_texture_cache(&mut self, dt: f32) {
        self.texture_cache.update(dt);
    }

    pub fn upload_texture(
        &mut self,
        device: &RenderDevice,
        resource_manager: &ResourceManager,
        texture: &ImageResource,
    ) -> Result<(), FrameworkError> {
        self.texture_cache.upload(device, resource_manager, texture)
    }

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
