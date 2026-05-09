use wgpu::{TextureAspect, TextureFormat, TextureUsages, TextureView, TextureViewDimension};

use crate::frame_graph::{
    ResourceHandle, ResourceRead, ResourceRef, ResourceWrite, TransientTexture,
};

#[derive(Clone, Default, Debug)]
pub struct TextureViewDescriptor {
    pub label: Option<String>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub usage: Option<TextureUsages>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}

impl TextureViewDescriptor {
    pub fn get_desc(&self) -> wgpu::TextureViewDescriptor<'_> {
        todo!()
    }
}

#[derive(Clone)]
pub struct TransientTextureViewDescriptor<ViewType> {
    pub texture: ResourceRef<TransientTexture, ViewType>,
    pub desc: TextureViewDescriptor,
}

#[derive(Clone)]
pub struct TransientTextureViewHandleDescriptor {
    pub texture: ResourceHandle<TransientTexture>,
    pub desc: TextureViewDescriptor,
}

#[derive(Clone)]
pub enum TransientTextureView {
    TextureView(TextureView),
    Read(TransientTextureViewDescriptor<ResourceRead>),
    Write(TransientTextureViewDescriptor<ResourceWrite>),
}

#[derive(Clone)]
pub enum TransientTextureViewHandle {
    TextureView(TextureView),
    Descriptor(TransientTextureViewHandleDescriptor),
}
