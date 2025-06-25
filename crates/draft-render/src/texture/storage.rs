use fyrox_core::log::Log;
use wgpu::util::DeviceExt;

use crate::{
    FrameworkError, TemporaryCache, Texture, TextureResource,
    gfx_base::Sampler,
    resource::{RenderDevice, RenderQueue, RenderTexture},
};

#[derive(Default)]
pub struct TextureStorage {
    pub texture_cache: TemporaryCache<TextureData>,
}

impl TextureStorage {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        queue: &RenderQueue,
        texture: &TextureResource,
    ) -> Option<&TextureData> {
        let mut texture_state = texture.state();

        if let Some(texture_state) = texture_state.data() {
            match self.texture_cache.get_mut_or_insert_with(
                &texture_state.cache_index,
                Default::default(),
                || TextureData::new(device, queue, texture_state),
            ) {
                Ok(data) => {
                    data.update(texture_state);
                    Some(data)
                }
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}

pub struct TextureData {
    pub sampler: Sampler,
    pub texture: RenderTexture,
}

fn get_texture_key(texture: &Texture) -> String {
    format!("texture_{}", texture.cache_index.get())
}

impl TextureData {
    pub fn update(&mut self, texture: &Texture) {
        self.texture.key = get_texture_key(texture);
    }

    pub fn new(
        device: &RenderDevice,
        queue: &RenderQueue,
        texture: &Texture,
    ) -> Result<Self, FrameworkError> {
        let raw_texture = device.wgpu_device().create_texture_with_data(
            queue.wgpu_queue(),
            texture.get_desc(),
            Default::default(),
            texture.as_bytes(),
        );

        let key = get_texture_key(texture);

        let render_texture = RenderTexture {
            key,
            value: raw_texture,
            desc: texture.image.texture_info.clone(),
        };

        let raw_sampler = device
            .wgpu_device()
            .create_sampler(texture.sampler_info.get_desc());

        let sampler = Sampler::new(raw_sampler, texture.sampler_info.info.clone());

        Ok(TextureData {
            texture: render_texture,
            sampler,
        })
    }
}
