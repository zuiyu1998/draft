use std::borrow::Cow;
use std::sync::Arc;

use crate::gfx_base::{BufferDescriptor, BufferInitDescriptor, GpuBuffer};
use wgpu::{BufferAddress, BufferUsages, COPY_BUFFER_ALIGNMENT};

use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcTransientResource,
    IntoArcTransientResource, TransientResource, TransientResourceDescriptor,
};

impl IntoArcTransientResource for TransientBuffer {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource {
        ArcTransientResource::Buffer(self)
    }
}

#[derive(Clone)]
pub struct TransientBuffer {
    pub resource: GpuBuffer,
    pub desc: TransientBufferDescriptor,
}

impl TransientResource for TransientBuffer {
    type Descriptor = TransientBufferDescriptor;

    fn borrow_resource(res: &AnyTransientResource) -> &Self {
        match res {
            AnyTransientResource::OwnedBuffer(res) => res,
            AnyTransientResource::ImportedBuffer(res) => res,
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
pub struct TransientBufferDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub size: BufferAddress,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

impl TransientBufferDescriptor {
    pub fn from_buffer_init_desc(desc: &BufferInitDescriptor) -> Self {
        if desc.contents.is_empty() {
            TransientBufferDescriptor {
                label: desc.label.as_ref().map(|label| label.to_string().into()),
                size: 0,
                usage: desc.usage,
                mapped_at_creation: false,
            }
        } else {
            let unpadded_size = desc.contents.len() as BufferAddress;
            // Valid vulkan usage is
            // 1. buffer size must be a multiple of COPY_BUFFER_ALIGNMENT.
            // 2. buffer size must be greater than 0.
            // Therefore we round the value up to the nearest multiple, and ensure it's at least COPY_BUFFER_ALIGNMENT.
            let align_mask = COPY_BUFFER_ALIGNMENT - 1;
            let padded_size =
                ((unpadded_size + align_mask) & !align_mask).max(COPY_BUFFER_ALIGNMENT);

            TransientBufferDescriptor {
                label: desc.label.as_ref().map(|label| label.to_string().into()),
                size: padded_size,
                usage: desc.usage,
                mapped_at_creation: false,
            }
        }
    }

    pub fn from_buffer_desc(desc: &BufferDescriptor) -> Self {
        Self {
            label: desc.label.clone(),
            size: desc.size,
            usage: desc.usage,
            mapped_at_creation: desc.mapped_at_creation,
        }
    }

    pub fn get_buffer_desc(&self) -> BufferDescriptor {
        BufferDescriptor {
            label: self.label.clone(),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation,
        }
    }
}

impl From<TransientBufferDescriptor> for AnyTransientResourceDescriptor {
    fn from(value: TransientBufferDescriptor) -> Self {
        AnyTransientResourceDescriptor::Buffer(value)
    }
}

impl TransientResourceDescriptor for TransientBufferDescriptor {
    type Resource = TransientBuffer;

    fn borrow_resource_descriptor(res: &AnyTransientResourceDescriptor) -> &Self {
        match res {
            AnyTransientResourceDescriptor::Buffer(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}
