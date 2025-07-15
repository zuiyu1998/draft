mod geometry;
mod material;
mod pipeline;
mod pipeline_descriptor_cache;
mod shader;
mod texture;

pub use geometry::*;
pub use material::*;
pub use pipeline::*;
pub use pipeline_descriptor_cache::*;
pub use shader::*;
pub use texture::*;

use crate::gfx_base::{RenderAdapter, RenderDevice, RenderInstance, RenderQueue};

pub struct RenderWorld {
    pub server: RenderServer,
    pub pipeline_cache: PipelineCache,
    pub geometry_cache: GeometryCache,
    pub texture_cache: TextureCache,
    pub material_cache: MaterialCache,
    pub pipeline_descriptor_cache: PipelineDescriptorCache,
}

impl RenderWorld {
    pub fn new(server: RenderServer) -> Self {
        Self {
            pipeline_cache: PipelineCache::new(server.device.clone()),
            server,
            geometry_cache: Default::default(),
            texture_cache: Default::default(),
            material_cache: Default::default(),
            pipeline_descriptor_cache: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pipeline_cache.process();
        self.texture_cache.update(dt);
    }

    pub fn update_texture(&mut self, texture: &TextureResource) {
        self.texture_cache.remove(texture);

        let _ = self
            .texture_cache
            .get_or_insert(&self.server.device, &self.server.queue, texture);
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}
