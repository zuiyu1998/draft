use wgpu::util::DeviceExt;

use crate::{
    BindGroupLayoutDescriptor, BufferInitDescriptor, GpuBindGroupLayout, GpuSampler,
    SamplerDescriptor,
};

use super::{BufferDescriptor, GpuBuffer, RawDevice};

#[derive(Clone)]
pub struct RenderDevice {
    device: RawDevice,
}

impl RenderDevice {
    pub fn new(device: RawDevice) -> Self {
        Self { device }
    }

    pub fn wgpu_device(&self) -> &RawDevice {
        &self.device
    }

    pub fn create_sampler(&self, desc: &SamplerDescriptor) -> GpuSampler {
        let sampler = self.device.create_sampler(&desc.get_desc());
        GpuSampler::new(sampler)
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer(&desc.get_desc());
        GpuBuffer::new(buffer)
    }

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer_init(&desc.to_buffer_init_desc());
        GpuBuffer::new(buffer)
    }

    pub fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> GpuBindGroupLayout {
        let entries = desc
            .entries
            .clone()
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        let bind_group_layout =
            self.wgpu_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });
        GpuBindGroupLayout::new(bind_group_layout)
    }
}
