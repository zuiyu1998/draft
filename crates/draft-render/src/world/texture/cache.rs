use wgpu::util::DeviceExt;

use crate::{
    FrameworkError, TemporaryCache, Texture, TextureResource,
    gfx_base::{RawTextureDescriptor, RenderDevice, RenderQueue, Sampler},
    render_resource::RenderTexture,
};

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
            match self.cache.get_mut_or_insert_with(
                &texture_state.cache_index,
                Default::default(),
                || TextureData::new(device, queue, texture_state),
            ) {
                Ok(data) => {
                    data.update(texture_state);
                    Ok(data)
                }
                Err(error) => Err(error),
            }
        } else {
            Err(texture.clone().into())
        }
    }
}

pub struct TextureData {
    pub render_data: TextureRenderData,
}

#[derive(Clone)]
pub struct TextureRenderData {
    pub sampler: Sampler,
    pub texture: RenderTexture,
}

fn get_texture_key(texture: &Texture) -> String {
    format!("texture_{}", texture.cache_index.get())
}

impl TextureData {
    pub fn update(&mut self, texture: &Texture) {
        self.render_data.texture.key = get_texture_key(texture);
    }

    pub fn new(
        device: &RenderDevice,
        queue: &RenderQueue,
        texture: &Texture,
    ) -> Result<Self, FrameworkError> {
        let texture_info = &texture.image.texture_info;

        let view_formats: Vec<_> = texture_info
            .view_formats
            .iter()
            .map(|format| (*format).into())
            .collect();

        let raw_texture = device.wgpu_device().create_texture_with_data(
            queue.wgpu_queue(),
            &RawTextureDescriptor {
                label: texture_info.label.as_deref(),
                size: texture_info.size.into(),
                mip_level_count: texture_info.mip_level_count,
                sample_count: texture_info.sample_count,
                dimension: texture_info.dimension.into(),
                format: texture_info.format.into(),
                usage: texture_info.usage.into(),
                view_formats: &view_formats,
            },
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
            .create_sampler(&texture.sampler_info.as_desc());

        let sampler = Sampler::new(raw_sampler, texture.sampler_info.info.clone());

        Ok(TextureData {
            render_data: TextureRenderData {
                sampler,
                texture: render_texture,
            },
        })
    }
}
