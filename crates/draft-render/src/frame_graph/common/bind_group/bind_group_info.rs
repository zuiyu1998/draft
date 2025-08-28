use std::{borrow::Cow, num::NonZero};

use crate::{
    frame_graph::{PassContext, TransientBuffer},
    gfx_base::{BindGroupDescriptor, GpuBindGroup, GpuBindGroupLayout, RawBufferBinding},
};

use super::BindGroupEntryInfo;

#[derive(Clone)]
pub struct BindGroupInfo {
    pub label: Option<Cow<'static, str>>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<BindGroupEntryInfo>,
}

pub struct BindGroup(GpuBindGroup);

impl BindGroup {
    pub fn get_gpu_bind_group(&self) -> &GpuBindGroup {
        &self.0
    }

    pub fn new(context: &PassContext<'_>, info: &BindGroupInfo) -> Self {
        let entries = vec![];

        let desc = BindGroupDescriptor {
            label: info.label.clone(),
            layout: info.layout.clone(),
            entries,
        };

        BindGroup(context.render_device.create_bind_group(&desc))
    }
}

pub enum BindingResource {
    Buffer {
        buffer: TransientBuffer,
        size: Option<NonZero<u64>>,
        offset: u64,
    },
    Sampler(wgpu::Sampler),
    TextureView(wgpu::TextureView),
    TextureViewArray(Vec<wgpu::TextureView>),
}

pub enum BindingResourceTemp<'a> {
    Buffer {
        buffer: &'a TransientBuffer,
        size: Option<NonZero<u64>>,
        offset: u64,
    },
    Sampler(wgpu::Sampler),
    TextureView(wgpu::TextureView),
    TextureViewArray(Vec<&'a wgpu::TextureView>),
}

impl BindingResourceTemp<'_> {
    pub fn get_resource_binding(&self) -> wgpu::BindingResource {
        match self {
            BindingResourceTemp::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler),
            BindingResourceTemp::TextureView(texture_view) => {
                wgpu::BindingResource::TextureView(texture_view)
            }
            BindingResourceTemp::Buffer {
                buffer,
                size,
                offset,
            } => wgpu::BindingResource::Buffer(RawBufferBinding {
                buffer: buffer.resource.get_buffer(),
                offset: *offset,
                size: *size,
            }),
            BindingResourceTemp::TextureViewArray(texture_views) => {
                wgpu::BindingResource::TextureViewArray(texture_views.as_slice())
            }
        }
    }
}
