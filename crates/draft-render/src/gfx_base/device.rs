use wgpu::util::DeviceExt;

use crate::gfx_base::BufferInitDescriptor;

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

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer(&desc.get_desc());
        GpuBuffer::new(buffer)
    }

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer_init(&desc.to_buffer_init_desc());
        GpuBuffer::new(buffer)
    }
}
