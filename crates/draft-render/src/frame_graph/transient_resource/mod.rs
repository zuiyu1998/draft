mod buffer;
mod cache;
mod texture;

pub use buffer::*;
pub use cache::*;
pub use texture::*;

use std::sync::Arc;

use crate::resource::RenderDevice;

pub trait TransientResourceCreator {
    fn create_resource(&self, desc: &AnyTransientResourceDescriptor) -> AnyTransientResource;
}

impl TransientResourceCreator for RenderDevice {
    fn create_resource(&self, desc: &AnyTransientResourceDescriptor) -> AnyTransientResource {
        match desc {
            AnyTransientResourceDescriptor::Texture(desc) => {
                let view_formats = desc
                    .view_formats
                    .iter()
                    .map(|format| (*format).into())
                    .collect::<Vec<_>>();

                let resource = self.wgpu_device().create_texture(&wgpu::TextureDescriptor {
                    label: desc.label.as_deref(),
                    size: desc.size.into(),
                    mip_level_count: desc.mip_level_count,
                    sample_count: desc.sample_count,
                    dimension: desc.dimension.into(),
                    format: desc.format.into(),
                    usage: desc.usage.into(),
                    view_formats: &view_formats,
                });
                TransientTexture {
                    resource,
                    desc: desc.clone(),
                }
                .into()
            }
            AnyTransientResourceDescriptor::Buffer(desc) => {
                let resource = self.wgpu_device().create_buffer(&desc.get_buffer_desc());
                TransientBuffer {
                    resource,
                    desc: desc.clone(),
                }
                .into()
            }
        }
    }
}

#[derive(Clone)]
pub enum ArcTransientResource {
    Buffer(Arc<TransientBuffer>),
    Texture(Arc<TransientTexture>),
}

impl ArcTransientResource {
    pub fn get_desc(&self) -> AnyTransientResourceDescriptor {
        match self {
            ArcTransientResource::Buffer(res) => {
                AnyTransientResourceDescriptor::Buffer(res.desc.clone())
            }
            ArcTransientResource::Texture(res) => {
                AnyTransientResourceDescriptor::Texture(res.desc.clone())
            }
        }
    }
}

pub trait IntoArcTransientResource: TransientResource {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource;
}

pub enum AnyTransientResource {
    OwnedBuffer(TransientBuffer),
    ImportedBuffer(Arc<TransientBuffer>),
    OwnedTexture(TransientTexture),
    ImportedTexture(Arc<TransientTexture>),
}

impl From<TransientBuffer> for AnyTransientResource {
    fn from(value: TransientBuffer) -> Self {
        AnyTransientResource::OwnedBuffer(value)
    }
}

impl From<Arc<TransientBuffer>> for AnyTransientResource {
    fn from(value: Arc<TransientBuffer>) -> Self {
        AnyTransientResource::ImportedBuffer(value)
    }
}

impl From<TransientTexture> for AnyTransientResource {
    fn from(value: TransientTexture) -> Self {
        AnyTransientResource::OwnedTexture(value)
    }
}

impl From<Arc<TransientTexture>> for AnyTransientResource {
    fn from(value: Arc<TransientTexture>) -> Self {
        AnyTransientResource::ImportedTexture(value)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum AnyTransientResourceDescriptor {
    Buffer(BufferInfo),
    Texture(TextureInfo),
}

pub trait TransientResource: 'static {
    type Descriptor: TransientResourceDescriptor;

    fn borrow_resource(res: &AnyTransientResource) -> &Self;

    fn get_desc(&self) -> &Self::Descriptor;
}

pub trait TransientResourceDescriptor:
    'static + Clone + Into<AnyTransientResourceDescriptor>
{
    type Resource: TransientResource;

    fn borrow_resource_descriptor(res: &AnyTransientResourceDescriptor) -> &Self;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}
