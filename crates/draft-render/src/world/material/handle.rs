use fyrox_core::ImmutableString;

use crate::{
    frame_graph::{Handle, TransientBuffer},
    gfx_base::GpuSampler,
    render_resource::RenderTexture,
};

pub enum MaterialResourceHandle {
    Texture(MaterialTextureHandle),
    Sampler(MaterialSamplerHandle),
    Buffer(MaterialBufferHandle),
}

pub struct MaterialBufferHandle {
    pub offset: u32,
    pub size: u32,
    pub handle: Handle<TransientBuffer>,
}

pub struct MaterialTextureHandle {
    pub texture: RenderTexture,
}

pub struct MaterialSamplerHandle {
    pub sampler: GpuSampler,
}

pub struct MaterialPropertyGroupHandle {
    pub offset: u32,
    pub property_group_key: ImmutableString,
    pub size: u32,
}
