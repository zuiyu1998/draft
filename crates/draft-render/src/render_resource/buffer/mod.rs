use std::{
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use crate::gfx_base::{BufferAddress, RawBuffer};

use crate::frame_graph::{BufferInfo, FrameGraph, Handle, ResourceMaterial, TransientBuffer};

pub struct RenderBuffer {
    pub key: String,
    pub value: RawBuffer,
    pub desc: BufferInfo,
}

impl RenderBuffer {
    pub fn slice(&self, bounds: impl RangeBounds<BufferAddress>) -> BufferSlice {
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
        BufferSlice {
            offset,
            size,
            value: self.value.slice(bounds),
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
