use std::num::NonZero;

use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, TransientBuffer, TransientTexture},
    gfx_base::{
        BindGroupDescriptor, BindGroupLayout, BufferBinding, GpuBindGroup, GpuBindGroupEntry,
        GpuBindingResource, Sampler, TextureViewDescriptor,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransientBindGroupBuffer {
    pub buffer: Ref<TransientBuffer, ResourceRead>,
    pub size: Option<NonZero<u64>>,
    pub offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransientBindGroupTextureView {
    pub texture: Ref<TransientTexture, ResourceRead>,
    pub texture_view_desc: TextureViewDescriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransientBindGroupResource {
    Buffer(TransientBindGroupBuffer),
    Sampler(Sampler),
    TextureView(TransientBindGroupTextureView),
    TextureViewArray(Vec<TransientBindGroupTextureView>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransientBindGroupEntry {
    pub binding: u32,
    pub resource: TransientBindGroupResource,
}

impl TransientBindGroupEntry {
    pub fn get_gpu_bind_group_entry(&self, context: &PassContext<'_>) -> GpuBindGroupEntry {
        match &self.resource {
            TransientBindGroupResource::Buffer(binding) => {
                let buffer = context.resource_table().get_resource(&binding.buffer);

                GpuBindGroupEntry {
                    binding: self.binding,
                    resource: GpuBindingResource::Buffer(BufferBinding {
                        buffer: buffer.resource.clone(),
                        offset: binding.offset,
                        size: binding.size,
                    }),
                }
            }
            TransientBindGroupResource::Sampler(sampler) => GpuBindGroupEntry {
                binding: self.binding,
                resource: GpuBindingResource::Sampler(sampler.value().clone()),
            },
            TransientBindGroupResource::TextureView(binding) => {
                let texture = context.resource_table().get_resource(&binding.texture);
                let texture_view = texture.resource.create_view(&binding.texture_view_desc);

                GpuBindGroupEntry {
                    binding: self.binding,
                    resource: GpuBindingResource::TextureView(texture_view),
                }
            }
            TransientBindGroupResource::TextureViewArray(bindings) => {
                let bindings = bindings
                    .iter()
                    .map(|binding| {
                        let texture = context.resource_table().get_resource(&binding.texture);
                        texture.resource.create_view(&binding.texture_view_desc)
                    })
                    .collect();

                GpuBindGroupEntry {
                    binding: self.binding,
                    resource: GpuBindingResource::TextureViewArray(bindings),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TransientBindGroup {
    pub label: Option<String>,
    pub layout: BindGroupLayout,
    pub entries: Vec<TransientBindGroupEntry>,
}

impl TransientBindGroup {
    pub fn create_bind_group(&self, context: &PassContext<'_>) -> GpuBindGroup {
        let entries = self
            .entries
            .iter()
            .map(|entry| entry.get_gpu_bind_group_entry(context))
            .collect();

        let desc = BindGroupDescriptor {
            label: self.label.clone(),
            layout: self.layout.value().clone(),
            entries,
        };

        context.render_device().create_bind_group(&desc)
    }
}
