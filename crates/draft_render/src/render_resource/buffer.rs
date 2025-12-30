use std::{ops::RangeBounds, sync::Arc};

use draft_graphics::{
    frame_graph::{
        FrameGraph, Handle, ResourceMaterial, TransientBuffer, TransientBufferDescriptor,
    },
    gfx_base::{BufferDescriptor, GpuBuffer},
    wgpu::BufferAddress,
};

pub struct BufferSlice<'a> {
    pub offset: BufferAddress,
    pub size: BufferAddress,
    pub buffer: &'a Buffer,
}

pub struct Buffer {
    key: String,
    value: GpuBuffer,
    desc: BufferDescriptor,
}

impl Buffer {
    pub fn new(key: String, value: GpuBuffer, desc: BufferDescriptor) -> Self {
        Self { key, value, desc }
    }

    pub fn value(&self) -> &GpuBuffer {
        &self.value
    }

    pub fn slice(&self, bounds: impl RangeBounds<BufferAddress>) -> BufferSlice<'_> {
        let slice = self.value.slice(bounds);

        BufferSlice {
            offset: slice.offset,
            size: slice.size,
            buffer: self,
        }
    }
}

impl ResourceMaterial for Buffer {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<Self::ResourceType> {
        frame_graph.import(
            &self.key,
            Arc::new(TransientBuffer {
                resource: self.value.clone(),
                desc: TransientBufferDescriptor::from_buffer_desc(&self.desc),
            }),
        )
    }
}
