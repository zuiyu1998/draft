use wgpu::{
    BindGroupEntry as WgpuBindGroupEntry, CommandEncoder, Device, ShaderModuleDescriptor,
    SurfaceConfiguration,
};

use crate::gfx_base::{
    GpuPipelineLayout, GpuRenderPipeline, GpuShaderModule, PipelineLayoutDescriptor,
};

use super::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BindingResource, BufferDescriptor,
    CommandEncoderDescriptor, GpuBindGroup, GpuBindGroupLayout, GpuBindingResource, GpuBuffer,
    GpuSampler, GpuSurface, GpuTexture, GpuTextureView, RenderPipelineDescriptor,
    SamplerDescriptor, TextureDescriptor,
};

#[derive(Debug, Clone)]
pub struct RenderDevice {
    device: Device,
}

impl RenderDevice {
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    pub fn configure_surface(&self, surface: &GpuSurface, config: &SurfaceConfiguration) {
        surface.get_wgpu_surface().configure(&self.device, config);
    }

    pub fn create_render_pipeline(&self, desc: RenderPipelineDescriptor) -> GpuRenderPipeline {
        let buffers = desc
            .vertex
            .buffers
            .iter()
            .map(|layout| layout.get_wgpu_vertex_buffer_layout())
            .collect::<Vec<_>>();

        let vertex = wgpu::VertexState {
            module: desc.vertex.module.get_wgpu_shader_module(),
            entry_point: desc.vertex.entry_point.as_deref(),
            compilation_options: desc.vertex.compilation_options,
            buffers: &buffers,
        };

        let fragment = desc.fragment.as_ref().map(|fragment| wgpu::FragmentState {
            module: fragment.module.get_wgpu_shader_module(),
            entry_point: fragment.entry_point.as_deref(),
            compilation_options: fragment.compilation_options.clone(),
            targets: &fragment.targets,
        });

        GpuRenderPipeline::new(
            self.device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: desc.label.as_deref(),
                    layout: desc
                        .layout
                        .as_ref()
                        .map(|layout| layout.value().get_wgpu_pipeline_layout()),
                    vertex,
                    primitive: desc.primitive,
                    depth_stencil: desc.depth_stencil,
                    multisample: desc.multisample,
                    fragment,
                    multiview: None,
                    cache: None,
                }),
        )
    }

    pub fn create_shader_module(&self, desc: ShaderModuleDescriptor) -> GpuShaderModule {
        GpuShaderModule::new(self.device.create_shader_module(desc))
    }

    pub fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> GpuPipelineLayout {
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|bind_group_layout| bind_group_layout.get_wgpu_bind_group_layout())
            .collect::<Vec<_>>();

        GpuPipelineLayout::new(self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: desc.label.as_deref(),
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &desc.push_constant_ranges,
            },
        ))
    }

    pub fn create_sampler(&self, desc: &SamplerDescriptor) -> GpuSampler {
        let sampler = self.device.create_sampler(&desc.get_wgpu_desc());
        GpuSampler::new(sampler)
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
                    BindingResource::Sampler(binding.get_wgpu_sampler()),
                ),
                GpuBindingResource::SamplerArray(ref bindings) => (
                    entry.binding,
                    BindingResource::SamplerArray(
                        bindings.iter().map(GpuSampler::get_wgpu_sampler).collect(),
                    ),
                ),
                GpuBindingResource::TextureView(ref binding) => (
                    entry.binding,
                    BindingResource::TextureView(binding.get_wgpu_texture_view()),
                ),
                GpuBindingResource::TextureViewArray(ref bindings) => (
                    entry.binding,
                    BindingResource::TextureViewArray(
                        bindings
                            .iter()
                            .map(GpuTextureView::get_wgpu_texture_view)
                            .collect(),
                    ),
                ),
            })
            .collect::<Vec<_>>();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: desc.label.as_deref(),
            layout: desc.layout.get_wgpu_bind_group_layout(),
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

    pub fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> GpuBindGroupLayout {
        let entries = desc.entries.clone();

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });
        GpuBindGroupLayout::new(bind_group_layout)
    }

    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor) -> CommandEncoder {
        self.device.create_command_encoder(&desc.get_buffer_desc())
    }

    pub fn create_texture(&self, desc: &TextureDescriptor) -> GpuTexture {
        let texture = self.device.create_texture(&desc.get_wgpu_desc());
        GpuTexture::new(texture)
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer(&desc.get_wgpu_desc());
        GpuBuffer::new(buffer)
    }
}
