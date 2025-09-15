use std::{
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use draft_gfx_base::{BufferAddress, GpuBuffer};

use crate::frame_graph::{BufferInfo, FrameGraph, Handle, ResourceMaterial, TransientBuffer};

#[derive(Clone)]
pub struct RenderBuffer {
    pub key: String,
    pub value: GpuBuffer,
    pub desc: BufferInfo,
}

impl RenderBuffer {
    pub fn slice<'a>(&'a self, bounds: impl RangeBounds<BufferAddress>) -> BufferSlice<'a> {
        let offset = match bounds.start_bound() {
            Bound::Included(&bound) => bound,
            Bound::Excluded(&bound) => bound + 1,
            Bound::Unbounded => 0,
        };
        let size = match bounds.end_bound() {
            Bound::Included(&bound) => bound + 1,
            Bound::Excluded(&bound) => bound,
            Bound::Unbounded => self.value.get_buffer().size(),
        } - offset;
        BufferSlice {
            offset,
            size,
            value: self.value.get_buffer().slice(bounds),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BufferSlice<'a> {
    pub offset: wgpu::BufferAddress,
    pub value: wgpu::BufferSlice<'a>,
    pub size: wgpu::BufferAddress,
}

impl ResourceMaterial for RenderBuffer {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<Self::ResourceType> {
        let resource = Arc::new(TransientBuffer {
            resource: self.value.clone(),
            desc: self.desc.clone(),
        });

        frame_graph.import(&self.key, resource)
    }
}
