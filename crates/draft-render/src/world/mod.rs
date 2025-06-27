mod geometry;
mod material;
mod shader;
mod texture;

use std::sync::mpsc::Receiver;

pub use geometry::*;
pub use material::*;
pub use shader::*;
pub use texture::*;

use crate::gfx_base::{RenderAdapter, RenderDevice, RenderInstance, RenderQueue};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};

#[derive(Default)]
pub struct RenderStorage {
    material_storage: MaterialStorage,
    geometry_storage: GeometryStorage,
    texture_storage: TextureStorage,
}

impl RenderStorage {
    pub fn update_texture(&mut self, dt: f32) {
        self.texture_storage.update(dt);
    }

    pub fn material_storage(&self) -> &MaterialStorage {
        &self.material_storage
    }
}

pub struct RenderWorld {
    server: RenderServer,
    pub render_storage: RenderStorage,
    texture_event_receiver: Receiver<ResourceEvent>,
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
            server,
            render_storage: Default::default(),
            texture_event_receiver,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_texture(dt);
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

    fn update_texture(&mut self, dt: f32) {
        while let Ok(event) = self.texture_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(texture) = resource.try_cast::<Texture>() {
                    self.remove_texture(&texture);
                    let _ = self.get_texture_data(&texture);
                }
            }
        }

        self.render_storage.update_texture(dt);
    }

    fn remove_texture(&mut self, texture: &TextureResource) {
        self.render_storage.texture_storage.remove(texture);
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
    pub instance: RenderInstance,
    pub adapter: RenderAdapter,
}
