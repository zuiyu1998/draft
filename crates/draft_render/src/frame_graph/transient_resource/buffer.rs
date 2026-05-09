use std::sync::Arc;

use wgpu::Buffer;

use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcAnyTransientResource,
    IntoArcAnyTransientResource, TransientResource, TransientResourceDescriptor,
};

impl IntoArcAnyTransientResource for TransientBuffer {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcAnyTransientResource {
        ArcAnyTransientResource::Buffer(self)
    }
}

#[derive(Clone)]
pub struct TransientBuffer {
    pub resource: Buffer,
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
pub enum TransientBufferDescriptor {
    External,
    Manual(ManualBufferDescriptor),
}

impl TransientBufferDescriptor {
    pub fn get_desc(&self) -> wgpu::BufferDescriptor<'_> {
        todo!()
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ManualBufferDescriptor {
    pub label: Option<String>,
    pub size: wgpu::BufferAddress,
    pub usage: wgpu::BufferUsages,
}
