use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcAnyTransientResource,
    IntoArcAnyTransientResource, TransientResource, TransientResourceDescriptor,
};
use std::sync::Arc;
use wgpu::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

impl IntoArcAnyTransientResource for TransientTexture {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcAnyTransientResource {
        ArcAnyTransientResource::Texture(self)
    }
}

pub struct TransientTexture {
    pub resource: wgpu::Texture,
    pub desc: TransientTextureDescriptor,
}

impl TransientResource for TransientTexture {
    type Descriptor = TransientTextureDescriptor;

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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum TransientTextureDescriptor {
    External,
    Manual(ManualTextureDescriptor),
}

impl TransientTextureDescriptor {
    pub fn get_desc(&self) -> wgpu::TextureDescriptor<'_> {
        todo!()
    }
}

impl From<TransientTextureDescriptor> for AnyTransientResourceDescriptor {
    fn from(value: TransientTextureDescriptor) -> Self {
        AnyTransientResourceDescriptor::Texture(value)
    }
}

impl TransientResourceDescriptor for TransientTextureDescriptor {
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ManualTextureDescriptor {
    pub label: Option<String>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
}
