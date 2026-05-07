pub struct GpuBuffer(wgpu::Buffer);

impl GpuBuffer {
    pub fn new(buffer: wgpu::Buffer) -> Self {
        GpuBuffer(buffer)
    }

    pub fn value(&self) -> &wgpu::Buffer {
        &self.0
    }
}
