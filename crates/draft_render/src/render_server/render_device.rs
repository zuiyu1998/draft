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
        let desc = wgpu_utils::convert_texture_descriptor(desc);
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: desc.label,
            size: desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count: desc.mip_level_count,
            dimension: desc.dimension,
            format: desc.format,
            usage: desc.usage,
            view_formats: &desc.view_formats,
        })
    }

    pub fn create_sampler(&self, desc: &SamplerDescriptor) -> wgpu::Sampler {
        let desc = wgpu_utils::convert_sampler_descriptor(desc);

        self.device.create_sampler(&desc)
    }
}
