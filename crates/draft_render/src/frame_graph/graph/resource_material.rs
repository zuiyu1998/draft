use crate::frame_graph::{FrameGraph, ResourceHandle, TransientResource};

pub trait ResourceMaterial {
    type ResourceType: TransientResource;

    fn imported(&self, frame_graph: &mut FrameGraph) -> ResourceHandle<Self::ResourceType>;
}
