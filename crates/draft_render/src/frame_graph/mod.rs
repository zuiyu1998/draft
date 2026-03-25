mod bind_group;
mod handle;
mod pass;
mod pass_node;
mod pipeline_container;
mod resource_node;
mod resource_table;
mod texture_view;
mod transient_resource;

pub use bind_group::*;
pub use handle::*;
pub use pass::*;
pub use pass_node::*;
pub use pipeline_container::*;
pub use resource_node::*;
pub use resource_table::*;
pub use texture_view::*;
pub use transient_resource::*;

use crate::render_server::RenderDevice;
use wgpu::{BindGroup, BindGroupEntry};

pub trait TransientResourceCreator {
    fn create_resource(&self, desc: &AnyTransientResourceDescriptor) -> AnyTransientResource;
    fn create_bind_group(&self, desc: &TransientBindGroupDescriptor) -> BindGroup;
}

impl TransientResourceCreator for RenderDevice {
    fn create_bind_group(&self, desc: &TransientBindGroupDescriptor) -> BindGroup {
        let entries = desc
            .entries
            .iter()
            .map(|entry| match entry.resource {
                GpuBindingResource::Buffer(ref binding) => (
                    entry.binding,
                    TransientBindingResource::Buffer(binding.get_binding()),
                ),
                GpuBindingResource::BufferArray(ref bindings) => (
                    entry.binding,
                    TransientBindingResource::BufferArray(
                        bindings
                            .iter()
                            .map(|binding| binding.get_binding())
                            .collect(),
                    ),
                ),
                GpuBindingResource::Sampler(ref binding) => {
                    (entry.binding, TransientBindingResource::Sampler(binding))
                }
                GpuBindingResource::SamplerArray(ref bindings) => (
                    entry.binding,
                    TransientBindingResource::SamplerArray(bindings.iter().collect()),
                ),
                GpuBindingResource::TextureView(ref binding) => (
                    entry.binding,
                    TransientBindingResource::TextureView(binding),
                ),
                GpuBindingResource::TextureViewArray(ref bindings) => (
                    entry.binding,
                    TransientBindingResource::TextureViewArray(bindings.iter().collect()),
                ),
            })
            .collect::<Vec<_>>();

        self.wgpu_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: desc.label.as_deref(),
                layout: &desc.layout,
                entries: &entries
                    .iter()
                    .map(|(binding, resource)| BindGroupEntry {
                        binding: *binding,
                        resource: resource.get_binding_resource(),
                    })
                    .collect::<Vec<_>>(),
            })
    }

    fn create_resource(&self, desc: &AnyTransientResourceDescriptor) -> AnyTransientResource {
        match desc {
            AnyTransientResourceDescriptor::Texture(desc) => {
                let resource = self.wgpu_device().create_texture(&desc.get_desc());
                TransientTexture {
                    resource,
                    desc: desc.clone(),
                }
                .into()
            }
            AnyTransientResourceDescriptor::Buffer(desc) => {
                let resource = self.wgpu_device().create_buffer(&desc.get_desc());
                TransientBuffer {
                    resource,
                    desc: desc.clone(),
                }
                .into()
            }
        }
    }
}
