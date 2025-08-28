use std::borrow::Cow;

use crate::{
    BufferAddress, BufferSize, GpuBindGroupLayout, GpuBuffer, GpuSampler, GpuTextureView,
    WgpuBindGroup, WgpuBindingResource, WgpuBufferBinding, WgpuSampler, WgpuTextureView,
};

pub struct BindGroupDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<GpuBindGroupEntry>,
}

pub struct BufferBinding {
    pub buffer: GpuBuffer,
    pub offset: BufferAddress,
    pub size: Option<BufferSize>,
}

impl BufferBinding {
    pub fn get_binding(&self) -> WgpuBufferBinding {
        WgpuBufferBinding {
            buffer: self.buffer.get_buffer(),
            size: self.size,
            offset: self.offset,
        }
    }
}

pub enum GpuBindingResource {
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
    Sampler(GpuSampler),
    SamplerArray(Vec<GpuSampler>),
    TextureView(GpuTextureView),
    TextureViewArray(Vec<GpuTextureView>),
}

pub enum BindingResource<'a> {
    Buffer(WgpuBufferBinding<'a>),
    BufferArray(Vec<WgpuBufferBinding<'a>>),
    Sampler(&'a WgpuSampler),
    SamplerArray(Vec<&'a WgpuSampler>),
    TextureView(&'a WgpuTextureView),
    TextureViewArray(Vec<&'a WgpuTextureView>),
}

impl<'a> BindingResource<'a> {
    pub fn get_binding_resource(&self) -> WgpuBindingResource {
        match &self {
            BindingResource::Buffer(v) => WgpuBindingResource::Buffer(v.clone()),
            BindingResource::BufferArray(v) => WgpuBindingResource::BufferArray(v),
            BindingResource::Sampler(v) => WgpuBindingResource::Sampler(v),
            BindingResource::SamplerArray(v) => WgpuBindingResource::SamplerArray(v),
            BindingResource::TextureView(v) => WgpuBindingResource::TextureView(v),
            BindingResource::TextureViewArray(v) => WgpuBindingResource::TextureViewArray(v),
        }
    }
}

pub struct GpuBindGroupEntry {
    pub binding: u32,
    pub resource: GpuBindingResource,
}

#[derive(Debug, Clone)]
pub struct GpuBindGroup(WgpuBindGroup);

impl GpuBindGroup {
    pub fn new(bind_group: WgpuBindGroup) -> Self {
        Self(bind_group)
    }

    pub fn get_bind_group(&self) -> &WgpuBindGroup {
        &self.0
    }
}
