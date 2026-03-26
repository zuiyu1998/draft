use draft_graphics::{SamplerDescriptor, TextureDescriptor, wgpu_utils};

#[derive(Clone)]
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

    pub fn create_texture(&self, desc: &TextureDescriptor) -> wgpu::Texture {
        let desc = wgpu_utils::covert_texture_descriptor(desc);
        self.device.create_texture(&desc)
    }

    pub fn create_sampler(&self, _desc: &SamplerDescriptor) -> wgpu::Sampler {
        todo!()
    }
}
