use std::sync::Arc;

use crate::frame_graph::{FrameGraph, Handle, ResourceMaterial, TextureInfo, TransientTexture};

use draft_gfx_base::GpuTexture;

#[derive(Clone)]
pub struct RenderTexture {
    pub key: String,
    pub value: GpuTexture,
    pub desc: TextureInfo,
}

impl ResourceMaterial for RenderTexture {
    type ResourceType = TransientTexture;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<Self::ResourceType> {
        let resource = Arc::new(TransientTexture {
            resource: self.value.clone(),
            desc: self.desc.clone(),
        });

        frame_graph.import(&self.key, resource)
    }
}
