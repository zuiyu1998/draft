mod buffer_allocator;
mod geometry;
mod material;
mod pipeline;
mod resource_key;
mod shader;
mod texture;
mod uniform;

pub use buffer_allocator::*;
pub use geometry::*;
pub use material::*;
pub use pipeline::*;
pub use resource_key::*;
pub use shader::*;
pub use texture::*;
pub use uniform::*;

use crate::{
    FrameworkError,
    gfx_base::{RenderAdapter, RenderDevice, RenderInstance, RenderQueue},
};

pub struct RenderWorld {
    pub server: RenderServer,
    pub pipeline_cache: PipelineCache,
    pub geometry_cache: GeometryCache,
    texture_cache: TextureCache,
    pub uniform_buffer_cache: UniformBufferCache,
}

impl RenderWorld {
    pub fn new(server: RenderServer) -> Self {
        Self {
            pipeline_cache: PipelineCache::new(server.device.clone()),
            uniform_buffer_cache: UniformBufferCache::new(
                server.device.clone(),
                server.queue.clone(),
            ),
            server,
            geometry_cache: Default::default(),
            texture_cache: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pipeline_cache.process();
        self.texture_cache.update(dt);
    }

    pub fn get_or_create_texture(
        &mut self,
        texture: &TextureResource,
    ) -> Result<&TextureData, FrameworkError> {
        self.texture_cache
            .get_or_create(&self.server.device, &self.server.queue, texture)
    }

    pub fn update_texture(&mut self, texture: &TextureResource) {
        self.texture_cache.remove(texture);

        let _ = self
            .texture_cache
            .get_or_create(&self.server.device, &self.server.queue, texture);
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}
