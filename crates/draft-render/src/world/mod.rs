mod geometry;
mod material;
mod pipeline;
mod shader;
mod texture;

use std::sync::mpsc::Receiver;

pub use geometry::*;
pub use material::*;
pub use pipeline::*;
pub use shader::*;
pub use texture::*;

use crate::gfx_base::{RenderAdapter, RenderDevice, RenderInstance, RenderQueue};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};

pub struct RenderWorld {
    pub server: RenderServer,
    texture_event_receiver: Receiver<ResourceEvent>,
    pub pipeline_cache: PipelineCache,
    pub geometry_cache: GeometryCache,
    pub texture_storage: TextureStorage,
    pub material_cache: MaterialCache,
}

impl RenderWorld {
    pub fn new(server: RenderServer, resource_manager: &ResourceManager) -> Self {
        let (texture_event_sender, texture_event_receiver) = std::sync::mpsc::channel();

        resource_manager
            .state()
            .event_broadcaster
            .add(texture_event_sender);

        resource_manager.add_loader(TextureLoader::default());

        Self {
            pipeline_cache: PipelineCache::new(server.device.clone()),
            server,
            geometry_cache: Default::default(),
            texture_storage: Default::default(),
            texture_event_receiver,
            material_cache: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_texture(dt);
    }

    fn update_texture(&mut self, dt: f32) {
        while let Ok(event) = self.texture_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(texture) = resource.try_cast::<Texture>() {
                    self.remove_texture(&texture);
                    let _ = self.texture_storage.get_or_insert(
                        &self.server.device,
                        &self.server.queue,
                        &texture,
                    );
                }
            }
        }

        self.texture_storage.update(dt);
    }

    fn remove_texture(&mut self, texture: &TextureResource) {
        self.texture_storage.remove(texture);
    }
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}
