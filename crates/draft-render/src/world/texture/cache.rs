use std::sync::Arc;

use fyrox_core::sparse::AtomicIndex;

use crate::{
    FrameworkError, TemporaryCache, Texture, TextureResource, frame_graph::TransientTexture,
    render_resource::RenderTexture,
};
use draft_gfx_base::{GpuSampler, RenderDevice, RenderQueue};

#[derive(Default)]
pub struct TextureCache {
    pub cache: TemporaryCache<TextureData>,
}

impl TextureCache {
    pub fn remove(&mut self, texture: &TextureResource) {
        let mut state = texture.state();
        if let Some(texture_state) = state.data() {
            self.cache.remove(&texture_state.cache_index);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get_or_create(
        &mut self,
        device: &RenderDevice,
        queue: &RenderQueue,
        texture: &TextureResource,
    ) -> Result<&TextureData, FrameworkError> {
        let mut texture_state = texture.state();

        if let Some(texture_state) = texture_state.data() {
            match self.cache.get_or_insert_with(
                &texture_state.cache_index,
                Default::default(),
                || TextureData::new(device, queue, texture_state),
            ) {
                Ok(data) => Ok(data),
                Err(error) => Err(error),
            }
        } else {
            drop(texture_state);

            Err(texture.clone().into())
        }
    }
}

pub struct TextureData {
    pub sampler: GpuSampler,
    texture: TransientTexture,
    cache_index: Arc<AtomicIndex>,
}

impl TextureData {
    pub fn get_texture(&self) -> RenderTexture {
        RenderTexture {
            key: get_texture_key(self.cache_index.get()),
            value: self.texture.resource.clone(),
            desc: self.texture.desc.clone(),
        }
    }
}

fn get_texture_key(index: usize) -> String {
    format!("texture_{index}")
}

impl TextureData {
    pub fn new(
        device: &RenderDevice,
        queue: &RenderQueue,
        texture: &Texture,
    ) -> Result<Self, FrameworkError> {
        let texture_info = &texture.image.texture_info;

        let raw_texture = device.create_texture_with_data(
            queue,
            &texture_info.get_desc(),
            Default::default(),
            texture.as_bytes(),
        );

        let transient_texture = TransientTexture {
            resource: raw_texture,
            desc: texture.image.texture_info.clone(),
        };

        let sampler = device.create_sampler(&texture.sampler_info.desc);

        Ok(TextureData {
            texture: transient_texture,
            sampler,
            cache_index: texture.cache_index.clone(),
        })
    }
}
