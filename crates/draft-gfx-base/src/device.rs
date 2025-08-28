use wgpu::util::DeviceExt;

use crate::{
    BindGroupLayoutDescriptor, BindingResource, BufferInitDescriptor, GpuBindGroup,
    GpuBindGroupLayout, GpuBindingResource, GpuPipelineLayout, GpuSampler, GpuTexture,
    PipelineLayoutDescriptor, RenderQueue, SamplerDescriptor, TextureDataOrder, TextureDescriptor,
    WgpuBindGroupEntry,
};

use super::{BindGroupDescriptor, BufferDescriptor, GpuBuffer, RawDevice};

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

    pub fn create_bind_group(&self, desc: &BindGroupDescriptor) -> GpuBindGroup {
        let entries = desc
            .entries
            .iter()
            .map(|entry| match entry.resource {
                GpuBindingResource::Buffer(ref binding) => (
                    entry.binding,
                    BindingResource::Buffer(binding.get_binding()),
                ),
                GpuBindingResource::BufferArray(ref bindings) => (
                    entry.binding,
                    BindingResource::BufferArray(
                        bindings
                            .iter()
                            .map(|binding| binding.get_binding())
                            .collect(),
                    ),
                ),
                GpuBindingResource::Sampler(ref binding) => (
                    entry.binding,
                    BindingResource::Sampler(binding.get_sampler()),
                ),
                GpuBindingResource::SamplerArray(ref bindings) => (
                    entry.binding,
                    BindingResource::SamplerArray(
                        bindings
                            .iter()
                            .map(|binding| binding.get_sampler())
                            .collect(),
                    ),
                ),
                GpuBindingResource::TextureView(ref binding) => (
                    entry.binding,
                    BindingResource::TextureView(binding.get_texture_view()),
                ),
                GpuBindingResource::TextureViewArray(ref bindings) => (
                    entry.binding,
                    BindingResource::TextureViewArray(
                        bindings
                            .iter()
                            .map(|binding| binding.get_texture_view())
                            .collect(),
                    ),
                ),
            })
            .collect::<Vec<_>>();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: desc.label.as_deref(),
            layout: desc.layout.get_bind_group_layout(),
            entries: &entries
                .iter()
                .map(|(binding, resource)| WgpuBindGroupEntry {
                    binding: *binding,
                    resource: resource.get_binding_resource(),
                })
                .collect::<Vec<_>>(),
        });

        GpuBindGroup::new(bind_group)
    }

    pub fn create_texture_with_data(
        &self,
        queue: &RenderQueue,
        desc: &TextureDescriptor,
        order: TextureDataOrder,
        data: &[u8],
    ) -> GpuTexture {
        let texture =
            self.device
                .create_texture_with_data(queue.wgpu_queue(), &desc.get_desc(), order, data);

        GpuTexture::new(texture)
    }

    pub fn create_sampler(&self, desc: &SamplerDescriptor) -> GpuSampler {
        let sampler = self.device.create_sampler(&desc.get_desc());
        GpuSampler::new(sampler)
    }

    pub fn create_texture(&self, desc: &TextureDescriptor) -> GpuTexture {
        let buffer = self.device.create_texture(&desc.get_desc());
        GpuTexture::new(buffer)
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer(&desc.get_desc());
        GpuBuffer::new(buffer)
    }

    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer_init(&desc.to_buffer_init_desc());
        GpuBuffer::new(buffer)
    }

    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> GpuPipelineLayout {
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|v| v.get_bind_group_layout())
            .collect::<Vec<_>>();

        let pipeline_layout =
            self.wgpu_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[],
                });
        GpuPipelineLayout::new(pipeline_layout)
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
