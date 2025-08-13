mod buffer;
mod geometry;
mod material;
mod pipeline;
mod shader;
mod texture;

pub use buffer::*;
pub use geometry::*;
pub use material::*;
pub use pipeline::*;
pub use shader::*;
pub use texture::*;

use crate::{
    FrameworkError,
    gfx_base::{RenderAdapter, RenderDevice, RenderInstance, RenderQueue},
};

pub struct RenderWorld {
    pub server: RenderServer,
    pub pipeline_cache: PipelineCache,
    pub geometry_cache: GeometryCache,
    pub texture_cache: TextureCache,
    pub buffer_cache: BufferCache,
    pub buffer_allocator: BufferAllocator,
    pub material_buffer_handle_cache: MaterialBufferHandleCache,
    pub material_effect_info_container: MaterialEffectInfoContainer,
}

impl RenderWorld {
    pub fn new(server: RenderServer) -> Self {
        Self {
            pipeline_cache: PipelineCache::new(server.device.clone()),
            buffer_cache: BufferCache::new(server.device.clone(), server.queue.clone()),
            server,
            geometry_cache: Default::default(),
            texture_cache: Default::default(),
            buffer_allocator: Default::default(),
            material_buffer_handle_cache: Default::default(),
            material_effect_info_container: MaterialEffectInfoContainer::default(),
        }
    }

    pub fn register_material<T: ErasedMaterial>(&mut self) {
        T::register_material_effects(&mut self.material_effect_info_container);
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
