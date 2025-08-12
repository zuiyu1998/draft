use std::borrow::Cow;

use crate::{BufferAddress, BufferUsages, RawBuffer, RawBufferDescriptor, RawBufferInitDescriptor};

#[derive(Clone)]
pub struct GpuBuffer(RawBuffer);

impl GpuBuffer {
    pub fn get_buffer(&self) -> &RawBuffer {
        &self.0
    }

    pub fn new(buffer: RawBuffer) -> Self {
        Self(buffer)
    }
}

pub struct BufferDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub size: BufferAddress,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

impl BufferDescriptor {
    pub fn get_desc(&self) -> RawBufferDescriptor {
        RawBufferDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation,
        }
    }
}

pub struct BufferInitDescriptor<'a> {
    pub label: Option<Cow<'static, str>>,
    pub usage: BufferUsages,
    pub contents: &'a [u8],
}

impl BufferInitDescriptor<'_> {
    pub fn to_buffer_desc(&self) -> BufferDescriptor {
        BufferDescriptor {
            label: self.label.clone(),
            size: self.contents.len() as u64,
            usage: self.usage,
            mapped_at_creation: true,
        }
    }

    pub fn to_buffer_init_desc(&self) -> RawBufferInitDescriptor {
        RawBufferInitDescriptor {
            label: self.label.as_deref(),
            usage: self.usage,
            contents: self.contents,
        }
    }
}
