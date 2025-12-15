use wgpu::{
    BindGroup as WgpuBindGroup, BindingResource as WgpuBindingResource, BufferAddress,
    BufferBinding as WgpuBufferBinding, BufferSize, Sampler as WgpuSampler,
    TextureView as WgpuTextureView,
};

use crate::gfx_base::{GpuBindGroupLayout, GpuBuffer, GpuSampler, GpuTextureView};

#[derive(Clone, Debug)]
pub struct BufferBinding {
    pub buffer: GpuBuffer,
    pub offset: BufferAddress,
    pub size: Option<BufferSize>,
}

impl BufferBinding {
    pub(crate) fn get_binding<'a>(&'a self) -> WgpuBufferBinding<'a> {
        WgpuBufferBinding {
            buffer: self.buffer.get_wgpu_buffer(),
            size: self.size,
            offset: self.offset,
        }
    }
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
    pub fn get_binding_resource(&'a self) -> WgpuBindingResource<'a> {
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

#[derive(Clone)]
pub enum GpuBindingResource {
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
    Sampler(GpuSampler),
    SamplerArray(Vec<GpuSampler>),
    TextureView(GpuTextureView),
    TextureViewArray(Vec<GpuTextureView>),
}

#[derive(Clone)]
pub struct GpuBindGroupEntry {
    pub binding: u32,
    pub resource: GpuBindingResource,
}

#[derive(Clone)]
pub struct BindGroupDescriptor {
    pub label: Option<String>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<GpuBindGroupEntry>,
}

#[derive(Debug, Clone)]
pub struct GpuBindGroup(WgpuBindGroup);

impl GpuBindGroup {
    pub fn new(bind_group: WgpuBindGroup) -> Self {
        Self(bind_group)
    }

    pub(crate) fn get_wgpu_bind_group(&self) -> &WgpuBindGroup {
        &self.0
    }
}
