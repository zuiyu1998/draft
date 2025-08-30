use std::num::NonZero;

use draft_gfx_base::{BufferBinding, GpuBindGroupEntry, GpuBindingResource};

use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, TransientBuffer, TransientTexture},
    gfx_base::{GpuSampler, TextureViewDescriptor},
};

#[derive(Clone)]
pub struct BindGroupEntryInfo {
    pub binding: u32,
    pub resource: BindGroupResourceBinding,
}

impl BindGroupEntryInfo {
    pub fn get_gpu_bind_group_entry(&self, context: &PassContext<'_>) -> GpuBindGroupEntry {
        match &self.resource {
            BindGroupResourceBinding::Buffer(binding) => {
                let buffer = context.resource_table.get_resource(&binding.buffer);

                GpuBindGroupEntry {
                    binding: self.binding,
                    resource: GpuBindingResource::Buffer(BufferBinding {
                        buffer: buffer.resource.clone(),
                        offset: binding.offset,
                        size: binding.size,
                    }),
                }
            }
            BindGroupResourceBinding::Sampler(sampler) => GpuBindGroupEntry {
                binding: self.binding,
                resource: GpuBindingResource::Sampler(sampler.clone()),
            },
            BindGroupResourceBinding::TextureView(binding) => {
                let texture = context.resource_table.get_resource(&binding.texture);
                let texture_view = texture.resource.create_view(&binding.texture_view_desc);

                GpuBindGroupEntry {
                    binding: self.binding,
                    resource: GpuBindingResource::TextureView(texture_view),
                }
            }
            BindGroupResourceBinding::TextureViewArray(bindings) => {
                let bindings = bindings
                    .iter()
                    .map(|binding| {
                        let texture = context.resource_table.get_resource(&binding.texture);
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
pub enum BindGroupResourceBinding {
    Buffer(BindGroupBufferBinding),
    Sampler(GpuSampler),
    TextureView(BindGroupTextureViewBinding),
    TextureViewArray(Vec<BindGroupTextureViewBinding>),
}

#[derive(Clone)]
pub struct BindGroupBufferBinding {
    pub buffer: Ref<TransientBuffer, ResourceRead>,
    pub size: Option<NonZero<u64>>,
    pub offset: u64,
}

#[derive(Clone)]
pub struct BindGroupTextureViewBinding {
    pub texture: Ref<TransientTexture, ResourceRead>,
    pub texture_view_desc: TextureViewDescriptor,
}

pub trait IntoBindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding;
}

impl IntoBindGroupResourceBinding for BindGroupBufferBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::Buffer(self)
    }
}

impl IntoBindGroupResourceBinding for &GpuSampler {
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::Sampler(self.clone())
    }
}

impl IntoBindGroupResourceBinding for BindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        self
    }
}

impl IntoBindGroupResourceBinding for &BindGroupResourceBinding {
    fn into_binding(self) -> BindGroupResourceBinding {
        self.clone()
    }
}

impl IntoBindGroupResourceBinding
    for (&Ref<TransientTexture, ResourceRead>, &TextureViewDescriptor)
{
    fn into_binding(self) -> BindGroupResourceBinding {
        BindGroupResourceBinding::TextureView(BindGroupTextureViewBinding {
            texture: self.0.clone(),
            texture_view_desc: self.1.clone(),
        })
    }
}
