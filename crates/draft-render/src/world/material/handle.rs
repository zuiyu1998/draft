use std::num::NonZero;

use draft_gfx_base::TextureViewDescriptor;

use crate::{
    frame_graph::{
        BindGroupBufferHandle, BindGroupBufferHandleHelper, BindGroupTextureViewHandle,
        BindGroupTextureViewHandleHelper, FrameGraph, ResourceMaterial,
    },
    gfx_base::GpuSampler,
    render_resource::{RenderBuffer, RenderTexture},
};

pub enum MaterialResourceHandle {
    Texture(MaterialTextureHandle),
    Sampler(MaterialSamplerHandle),
    Buffer(MaterialBufferHandle),
}

pub struct MaterialBufferHandle {
    pub offset: u32,
    pub size: Option<NonZero<u64>>,
    pub buffer: RenderBuffer,
}

impl BindGroupBufferHandleHelper for MaterialBufferHandle {
    fn make_bind_group_buffer_handle(&self, frame_graph: &mut FrameGraph) -> BindGroupBufferHandle {
        let buffer = self.buffer.imported(frame_graph);

        BindGroupBufferHandle {
            buffer,
            size: self.size,
            offset: self.offset as u64,
        }
    }
}

pub struct MaterialTextureHandle {
    pub texture: RenderTexture,
    pub texture_view_desc: TextureViewDescriptor,
}

impl BindGroupTextureViewHandleHelper for MaterialTextureHandle {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle {
        let texture = self.texture.imported(frame_graph);

        BindGroupTextureViewHandle {
            texture,
            texture_view_desc: self.texture_view_desc.clone(),
        }
    }
}

pub struct MaterialSamplerHandle {
    pub sampler: GpuSampler,
}
