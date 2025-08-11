use std::borrow::Cow;

use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, ResourceView, ResourceWrite, TransientTexture},
    gfx_base::{RawTextureView, TextureAspect, TextureFormat, TextureUsages, TextureViewDimension},
};

pub type TextureViewInfoRead = TextureViewInfo<ResourceRead>;

pub type TextureViewInfoWrite = TextureViewInfo<ResourceWrite>;

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
    pub fn get_texture_view_desc(&self) -> wgpu::TextureViewDescriptor {
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

pub struct TextureViewInfo<ViewType: ResourceView> {
    pub texture: Ref<TransientTexture, ViewType>,
    pub desc: TextureViewDescriptor,
}

impl<ViewType: ResourceView> Clone for TextureViewInfo<ViewType> {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            desc: self.desc.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TextureView(RawTextureView);

impl TextureView {
    pub fn new(texture_view: RawTextureView) -> Self {
        Self(texture_view)
    }

    pub fn get_gpu_texture_view(&self) -> &RawTextureView {
        &self.0
    }

    pub fn from_info<View: ResourceView>(
        context: &PassContext<'_>,
        info: &TextureViewInfo<View>,
    ) -> Self {
        TextureView::new(
            context
                .resource_table
                .get_resource(&info.texture)
                .resource
                .create_view(&info.desc.get_texture_view_desc()),
        )
    }
}
