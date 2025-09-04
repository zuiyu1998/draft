use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcTransientResource,
    IntoArcTransientResource, TransientResource, TransientResourceDescriptor,
};
use draft_gfx_base::{
    Extent3d, TextureDimension, TextureFormat, TextureUsages, WgpuTextureDescriptor,
};
use draft_gfx_base::{GpuTexture, TextureDescriptor};
use fyrox_core::{reflect::*, visitor::*};
use std::sync::Arc;

impl IntoArcTransientResource for TransientTexture {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource {
        ArcTransientResource::Texture(self)
    }
}

pub struct TransientTexture {
    pub resource: GpuTexture,
    pub desc: TextureInfo,
}

impl TransientResource for TransientTexture {
    type Descriptor = TextureInfo;

    fn borrow_resource(res: &AnyTransientResource) -> &Self {
        match res {
            AnyTransientResource::OwnedTexture(res) => res,
            AnyTransientResource::ImportedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        &self.desc
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Reflect, Visit, Default)]
pub struct TextureInfo {
    pub label: Option<String>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: Vec<TextureFormat>,
}

impl TextureInfo {
    pub fn from_texture_desc(desc: &WgpuTextureDescriptor) -> Self {
        TextureInfo {
            label: desc.label.map(|label| label.to_string()),
            size: desc.size.into(),
            mip_level_count: desc.mip_level_count,
            sample_count: desc.sample_count,
            dimension: desc.dimension.into(),
            format: desc.format.into(),
            usage: desc.usage.into(),
            view_formats: desc
                .view_formats
                .iter()
                .map(|format| (*format).into())
                .collect(),
        }
    }

    pub fn get_desc(&self) -> TextureDescriptor {
        TextureDescriptor {
            label: self.label.as_ref().map(|v| v.clone().into()),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: self.view_formats.clone(),
        }
    }
}

impl From<TextureInfo> for AnyTransientResourceDescriptor {
    fn from(value: TextureInfo) -> Self {
        AnyTransientResourceDescriptor::Texture(value)
    }
}

impl TransientResourceDescriptor for TextureInfo {
    type Resource = TransientTexture;

    fn borrow_resource_descriptor(res: &AnyTransientResourceDescriptor) -> &Self {
        match res {
            AnyTransientResourceDescriptor::Texture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}
