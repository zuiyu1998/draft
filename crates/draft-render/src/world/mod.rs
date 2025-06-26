mod geometry;
mod material;
mod shader;
mod texture;

pub use geometry::*;
pub use material::*;
pub use shader::*;
pub use texture::*;

use crate::gfx_base::{RenderDevice, RenderQueue};

#[derive(Default)]
pub struct RenderStorage {
    pub material_storage: MaterialStorage,
    pub geometry_storage: GeometryStorage,
    pub texture_storage: TextureStorage,
}

pub struct RenderWorld {
    server: RenderServer,
    pub render_storage: RenderStorage,
}

impl RenderWorld {
    pub fn new(server: RenderServer, render_storage: RenderStorage) -> Self {
        Self {
            server,
            render_storage,
        }
    }

    pub fn server(&self) -> &RenderServer {
        &self.server
    }

    pub fn get_material_data(&mut self, material: &MaterialResource) -> Option<&MaterialData> {
        self.render_storage
            .material_storage
            .get(&self.server.device, material)
    }

    pub fn get_geometry_data(&mut self, geometry: &GeometryResource) -> Option<&GeometryData> {
        self.render_storage
            .geometry_storage
            .get(&self.server.device, geometry)
    }

    pub fn get_texture_data(&mut self, texture: &TextureResource) -> Option<&TextureData> {
        self.render_storage
            .texture_storage
            .get(&self.server.device, &self.server.queue, texture)
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
}
