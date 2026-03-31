mod material_cache;
mod mesh_cache;
mod pipeline_cache;
mod render_camera;
mod temporary_cache;
mod texture_cache;
mod window_surface;
mod window_surface_texture;
mod mesh_material_cache;

pub use material_cache::*;
pub use mesh_cache::*;
pub use pipeline_cache::*;
pub use render_camera::*;
pub use temporary_cache::*;
pub use texture_cache::*;
pub use window_surface::*;
pub use window_surface_texture::*;
pub use mesh_material_cache::*;

use crate::{FrameworkError, render_server::RenderDevice};
use draft_image::ImageResource;
use draft_material::{MaterialResource, PipelineResource};
use draft_mesh::MeshResource;
use fyrox_resource::manager::ResourceManager;
use std::mem::take;
use draft_core::ResourceId;

#[derive(Default)]
pub struct RenderWorld {
    pub(crate) window_surface_textures: WindowSurfaceTextures,
    pub(crate) texture_cache: TextureCache,
    pub(crate) mesh_cache: MeshCache,
    pub(crate) pipeline_cache: PipelineCache,
    pub(crate) material_cache: MaterialCache,
}

impl RenderWorld {
    pub fn get_or_create_material_resource_id(
        &mut self,
        material: &MaterialResource,
    ) -> Option<ResourceId> {
        self.material_cache.get_or_create_resource_id(material)
    }

    pub fn update_material_cache(&mut self, dt: f32) {
        self.material_cache.update(dt);
    }

    pub fn upload_pipeline(&mut self, pipeline: &PipelineResource) {
        self.pipeline_cache.get(pipeline);
    }

    pub fn update_pipeline_cache(&mut self, dt: f32) {
        self.pipeline_cache.update(dt);
    }

    pub fn get_or_create_mesh_resource_id(&mut self, mesh: &MeshResource) -> Option<ResourceId> {
        self.mesh_cache.get_or_create_resource_id(mesh)
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
