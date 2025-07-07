use std::borrow::Cow;

use crate::{
    frame_graph::{
        FrameGraphContext, Ref, ResourceRead, ResourceView, ResourceWrite, TransientTexture,
    },
    gfx_base::{RawTextureView, TextureAspect, TextureFormat, TextureUsages, TextureViewDimension},
};

use super::TransientResourceBinding;

pub type TextureViewRead = TextureView<ResourceRead>;

pub type TextureViewWrite = TextureView<ResourceWrite>;

#[derive(Default, Clone, Debug)]
pub struct TextureViewInfo {
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

impl From<wgpu::TextureViewDescriptor<'_>> for TextureViewInfo {
    fn from(value: wgpu::TextureViewDescriptor) -> Self {
        TextureViewInfo {
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

impl TextureViewInfo {
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

pub struct TextureView<ViewType: ResourceView> {
    pub texture: Ref<TransientTexture, ViewType>,
    pub desc: TextureViewInfo,
}

impl<ViewType: ResourceView> Clone for TextureView<ViewType> {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            desc: self.desc.clone(),
        }
    }
}

impl<ViewType: ResourceView> TransientResourceBinding for TextureView<ViewType> {
    type Resource = RawTextureView;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource {
        frame_graph_context
            .get_resource(&self.texture)
            .resource
            .create_view(&self.desc.get_texture_view_desc())
    }
}
