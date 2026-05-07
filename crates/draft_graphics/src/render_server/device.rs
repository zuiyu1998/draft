use crate::GpuBuffer;

#[derive(Clone)]
pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn new(device: wgpu::Device) -> Self {
        Self { device }
    }

    pub fn create_gpu_buffer(&self, desc: &wgpu::BufferDescriptor) -> GpuBuffer {
        GpuBuffer::new(self.device.create_buffer(desc))
    }
}
