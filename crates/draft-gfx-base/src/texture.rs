use std::borrow::Cow;

use crate::{
    Extent3d, TextureAspect, TextureDimension, TextureFormat, TextureUsages, TextureViewDimension,
    WgpuTexture,
};

pub use wgpu::{
    TextureDescriptor as WgpuTextureDescriptor, TextureView as WgpuTextureView,
    TextureViewDescriptor as WgpuTextureViewDescriptor,
};

#[derive(Default, Clone, Debug)]
pub struct TextureViewDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub usage: Option<TextureUsages>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}

impl From<wgpu::TextureViewDescriptor<'_>> for TextureViewDescriptor {
    fn from(value: wgpu::TextureViewDescriptor) -> Self {
        TextureViewDescriptor {
            label: value
                .label
                .map(|label| label.to_string())
                .map(|label| label.into()),
            format: value.format.map(|format| format.into()),
            dimension: value.dimension.map(|dimension| dimension.into()),
            usage: value.usage.map(|usage| usage.into()),
            aspect: value.aspect,
            base_mip_level: value.base_mip_level,
            mip_level_count: value.mip_level_count,
            base_array_layer: value.base_array_layer,
            array_layer_count: value.array_layer_count,
        }
    }
}

impl TextureViewDescriptor {
    pub fn get_desc(&self) -> WgpuTextureViewDescriptor {
        wgpu::TextureViewDescriptor {
            label: self.label.as_deref(),
            format: self.format.map(|format| format.into()),
            dimension: self.dimension.map(|dimension| dimension.into()),
            usage: self.usage.map(|usage| usage.into()),
            aspect: self.aspect,
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count,
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count,
        }
    }
}

#[derive(Clone)]
pub struct GpuTextureView(WgpuTextureView);

impl GpuTextureView {
    pub fn get_texture_view(&self) -> &WgpuTextureView {
        &self.0
    }

    pub fn new(texture_view: WgpuTextureView) -> Self {
        Self(texture_view)
    }
}

#[derive(Clone)]
pub struct GpuTexture(WgpuTexture);

impl GpuTexture {
    pub fn get_texture(&self) -> &WgpuTexture {
        &self.0
    }

    pub fn new(texture: WgpuTexture) -> Self {
        Self(texture)
    }

    pub fn create_view(&self, desc: &TextureViewDescriptor) -> GpuTextureView {
        GpuTextureView::new(self.0.create_view(&desc.get_desc()))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct TextureDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: Vec<TextureFormat>,
}

impl TextureDescriptor {
    pub fn get_desc(&self) -> WgpuTextureDescriptor {
        todo!()
    }
}
