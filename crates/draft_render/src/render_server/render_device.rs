pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn new(device: wgpu::Device) -> Self {
        Self { device }
    }

    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
}
