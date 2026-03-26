use std::{borrow::Cow, time::Duration};

use draft_image::{Image, ImageResource};
use fyrox_core::Uuid;
use fyrox_resource::manager::ResourceManager;

use crate::{FrameworkError, render_resource::TemporaryCache, render_server::RenderDevice};

pub struct TextureRenderData {
    pub name: String,
    pub gpu_texture: wgpu::Texture,
    pub gpu_sampler: wgpu::Sampler,
    modifications_counter: u64,
    sampler_modifications_counter: u64,
}

fn create_gpu_texture(
    device: &RenderDevice,
    resource_manager: &ResourceManager,
    uuid: &Uuid,
    texture: &Image,
) -> Result<TextureRenderData, FrameworkError> {
    let path = resource_manager
        .try_get_state(Duration::from_millis(1))
        .and_then(|state| state.uuid_to_resource_path(*uuid));
    let name = path
        .as_ref()
        .map(|path| path.to_string_lossy())
        .unwrap_or_else(|| Cow::Borrowed(""));

    let gpu_texture = device.create_texture(&texture.texture_descriptor);
    let gpu_sampler = device.create_sampler(&texture.sampler_descriptor);

    Ok(TextureRenderData {
        name: name.to_string(),
        gpu_texture,
        gpu_sampler,
        modifications_counter: texture.modifications_count(),
        sampler_modifications_counter: texture.sampler_modifications_count(),
    })
}

#[derive(Default)]
pub struct TextureCache {
    cache: TemporaryCache<TextureRenderData>,
}

impl TextureCache {
    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn upload(
        &mut self,
        device: &RenderDevice,
        resource_manager: &ResourceManager,
        texture: &ImageResource,
    ) -> Result<(), FrameworkError> {
        let uuid = texture.resource_uuid();
        let texture = texture.state();
        if let Some(texture) = texture.data_ref() {
            self.cache.get_entry_mut_or_insert_with(
                &texture.cache_index,
                Default::default(),
                || create_gpu_texture(device, resource_manager, &uuid, texture),
            )?;
            Ok(())
        } else {
            Err(FrameworkError::Custom(
                "Texture is not loaded yet!".to_string(),
            ))
        }
    }

    pub fn get(
        &mut self,
        device: &RenderDevice,
        resource_manager: &ResourceManager,
        image_resource: &ImageResource,
    ) -> Option<&TextureRenderData> {
        let uuid = image_resource.resource_uuid();
        let texture_data_guard = image_resource.state();
        if let Some(texture) = texture_data_guard.data_ref() {
            match self.cache.get_mut_or_insert_with(
                &texture.cache_index,
                Default::default(),
                || create_gpu_texture(device, resource_manager, &uuid, texture),
            ) {
                Ok(entry) => {
                    // Check if some value has changed in resource.

                    // Data might change from last frame, so we have to check it and upload new if so.
                    let modifications_count = texture.modifications_count();
                    if entry.modifications_counter != modifications_count {
                        entry.gpu_texture = device.create_texture(&texture.texture_descriptor);
                    }

                    if entry.sampler_modifications_counter != texture.sampler_modifications_count()
                    {
                        entry.gpu_sampler = device.create_sampler(&texture.sampler_descriptor);
                    }

                    return Some(entry);
                }
                Err(_e) => {
                    drop(texture_data_guard);
                }
            }
        }
        None
    }
}
