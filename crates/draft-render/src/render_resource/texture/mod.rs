use std::sync::Arc;

use crate::{
    frame_graph::{
        BindGroupTextureViewHandle, BindGroupTextureViewHandleHelper, FrameGraph, Handle,
        ResourceMaterial, TextureInfo, TransientTexture,
    },
    gfx_base::{GpuTexture, TextureViewDescriptor},
};

#[derive(Clone)]
pub struct RenderTexture {
    pub key: String,
    pub value: GpuTexture,
    pub desc: TextureInfo,
}

impl BindGroupTextureViewHandleHelper for RenderTexture {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle {
        let texture = self.imported(frame_graph);

        BindGroupTextureViewHandle {
            texture,
            texture_view_desc: TextureViewDescriptor::default(),
        }
    }
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
