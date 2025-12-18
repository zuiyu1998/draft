use std::ops::{Bound, RangeBounds};

use wgpu::{
    Buffer as WgpuBuffer, BufferAddress, BufferDescriptor as WgpuBufferDescriptor, BufferUsages,
};

#[derive(Clone, Debug)]
pub struct GpuBuffer(WgpuBuffer);

impl GpuBuffer {
    pub(crate) fn get_wgpu_buffer(&self) -> &WgpuBuffer {
        &self.0
    }

    pub fn size(&self) -> u64 {
        self.0.size()
    }

    pub fn new(buffer: WgpuBuffer) -> Self {
        Self(buffer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferDescriptor {
    pub label: Option<String>,
    pub size: BufferAddress,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}

impl BufferDescriptor {
    pub fn get_wgpu_desc<'a>(&'a self) -> WgpuBufferDescriptor<'a> {
        WgpuBufferDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation,
        }
    }
}

pub struct BufferInitDescriptor<'a> {
    pub label: Option<String>,
    pub usage: BufferUsages,
    pub contents: &'a [u8],
}

impl BufferInitDescriptor<'_> {
    pub fn get_buffer_desc(&self) -> BufferDescriptor {
        BufferDescriptor {
            label: self.label.clone(),
            size: self.contents.len() as u64,
            usage: self.usage,
            mapped_at_creation: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BufferSlice {
    pub offset: wgpu::BufferAddress,
    pub size: wgpu::BufferAddress,
}

#[derive(Clone, Debug)]
pub struct Buffer {
    pub value: GpuBuffer,
    pub desc: BufferDescriptor,
}

impl Buffer {
    pub fn slice(&self, bounds: impl RangeBounds<wgpu::BufferAddress>) -> BufferSlice {
        // need to compute and store this manually because wgpu doesn't export offset and size on wgpu::BufferSlice
        let offset = match bounds.start_bound() {
            Bound::Included(&bound) => bound,
            Bound::Excluded(&bound) => bound + 1,
            Bound::Unbounded => 0,
        };
        let size = match bounds.end_bound() {
            Bound::Included(&bound) => bound + 1,
            Bound::Excluded(&bound) => bound,
            Bound::Unbounded => self.value.size(),
        } - offset;

        BufferSlice { offset, size }
    }
}
