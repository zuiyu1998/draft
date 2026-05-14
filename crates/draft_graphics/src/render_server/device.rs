use wgpu::util::{BufferInitDescriptor, DeviceExt};

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

    pub fn create_shader_module(&self, desc: wgpu::ShaderModuleDescriptor) -> wgpu::ShaderModule {
        self.device.create_shader_module(desc)
    }

    pub fn create_render_pipelie(
        &self,
        desc: &wgpu::RenderPipelineDescriptor,
    ) -> wgpu::RenderPipeline {
        self.device.create_render_pipeline(desc)
    }

    pub fn create_command_encoder(
        &self,
        desc: &wgpu::CommandEncoderDescriptor,
    ) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(desc)
    }

    pub fn create_gpu_buffer_init(&self, desc: &BufferInitDescriptor) -> wgpu::Buffer {
        self.device.create_buffer_init(desc)
    }

    pub fn create_gpu_buffer(&self, desc: &wgpu::BufferDescriptor) -> wgpu::Buffer {
        self.device.create_buffer(desc)
    }

    pub fn create_gpu_texture(&self, desc: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.device.create_texture(desc)
    }
}
