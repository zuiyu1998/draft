use std::sync::Arc;

use frame_graph::{BufferInfo, FrameGraph, Handle, ResourceMaterial, TransientBuffer, wgpu};

pub struct Buffer {
    pub key: String,
    pub value: wgpu::Buffer,
    pub desc: BufferInfo,
}

impl ResourceMaterial for Buffer {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<Self::ResourceType> {
        let resource = Arc::new(TransientBuffer {
            resource: self.value.clone(),
            desc: self.desc.clone(),
        });

        frame_graph.import(&self.key, resource)
    }
}
