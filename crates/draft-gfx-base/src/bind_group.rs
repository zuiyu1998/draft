use std::borrow::Cow;

use crate::{GpuBindGroupLayout, GpuSampler, WgpuBindGroup, WgpuBindGroupDescriptor};

pub struct BindGroupDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<BindGroupEntry>,
}

pub struct BufferBinding {}

pub enum BindingResource {
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
    Sampler(GpuSampler),
    SamplerArray(Vec<GpuSampler>),
}

pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}

impl BindGroupDescriptor {
    pub fn get_desc(&self) -> WgpuBindGroupDescriptor {
        todo!()
    }
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
