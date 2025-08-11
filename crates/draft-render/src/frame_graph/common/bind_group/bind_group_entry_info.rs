use std::num::NonZero;

use crate::{
    frame_graph::{Ref, ResourceRead, TextureViewDescriptor, TransientBuffer, TransientTexture},
    gfx_base::RawSampler,
};

#[derive(Clone)]
pub struct BindGroupEntryInfo {
    pub binding: u32,
    pub resource: BindGroupResourceBinding,
}

#[derive(Clone)]
pub enum BindGroupResourceBinding {
    Buffer(BindGroupBufferBinding),
    Sampler(RawSampler),
    TextureView(BindGroupTextureViewBinding),
    TextureViewArray(Vec<BindGroupTextureViewBinding>),
}

#[derive(Clone)]
pub struct BindGroupBufferBinding {
    pub buffer: Ref<TransientBuffer, ResourceRead>,
    pub size: Option<NonZero<u64>>,
    pub offset: u64,
}

#[derive(Clone)]
pub struct BindGroupTextureViewBinding {
    pub texture: Ref<TransientTexture, ResourceRead>,
    pub texture_view_desc: TextureViewDescriptor,
}

pub trait IntoBindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding;
}

impl IntoBindGroupResourceBinding for BindGroupBufferBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::Buffer(self)
    }
}

impl IntoBindGroupResourceBinding for &RawSampler {
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::Sampler(self.clone())
    }
}

impl IntoBindGroupResourceBinding for BindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        self
    }
}

impl IntoBindGroupResourceBinding for &BindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        self.clone()
    }
}

impl IntoBindGroupResourceBinding
    for (&Ref<TransientTexture, ResourceRead>, &TextureViewDescriptor)
{
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::TextureView(BindGroupTextureViewBinding {
            texture: self.0.clone(),
            texture_view_desc: self.1.clone(),
        })
    }
}
