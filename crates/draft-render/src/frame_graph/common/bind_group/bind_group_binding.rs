use std::{borrow::Cow, collections::HashMap, num::NonZero};

use wgpu::{BindGroupLayout, BufferBinding};

use crate::frame_graph::{RenderContext, ResourceBinding, TransientBuffer};

use super::{BindGroupEntryBinding, BindGroupResourceBinding};

#[derive(Clone)]
pub struct BindGroupBinding {
    pub label: Option<Cow<'static, str>>,
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntryBinding>,
}

pub enum BindingResource<'a> {
    Buffer {
        buffer: &'a TransientBuffer,
        size: Option<NonZero<u64>>,
        offset: u64,
    },
    Sampler(wgpu::Sampler),
    TextureView(wgpu::TextureView),
    TextureViewArray(Vec<wgpu::TextureView>),
}

pub enum BindingResourceTemp<'a> {
    Buffer {
        buffer: &'a TransientBuffer,
        size: Option<NonZero<u64>>,
        offset: u64,
    },
    Sampler(wgpu::Sampler),
    TextureView(wgpu::TextureView),
    TextureViewArray(Vec<&'a wgpu::TextureView>),
}

impl BindingResourceTemp<'_> {
    pub fn get_resource_binding(&self) -> wgpu::BindingResource {
        match self {
            BindingResourceTemp::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler),
            BindingResourceTemp::TextureView(texture_view) => {
                wgpu::BindingResource::TextureView(texture_view)
            }
            BindingResourceTemp::Buffer {
                buffer,
                size,
                offset,
            } => wgpu::BindingResource::Buffer(BufferBinding {
                buffer: &buffer.resource,
                offset: *offset,
                size: *size,
            }),
            BindingResourceTemp::TextureViewArray(texture_views) => {
                wgpu::BindingResource::TextureViewArray(texture_views.as_slice())
            }
        }
    }
}

impl ResourceBinding for BindGroupBinding {
    type Resource = wgpu::BindGroup;

    fn make_resource(&self, render_context: &RenderContext<'_>) -> Self::Resource {
        let mut resources = HashMap::new();

        for entry in self.entries.iter() {
            if let BindGroupResourceBinding::TextureViewArray(texture_view_refs) = &entry.resource {
                let mut texture_views = vec![];

                for texture_view_ref in texture_view_refs.iter() {
                    let texture = render_context.get_resource(&texture_view_ref.texture);

                    texture_views.push(
                        texture.resource.create_view(
                            &texture_view_ref.texture_view_info.get_texture_view_desc(),
                        ),
                    );
                }
                resources.insert(entry.binding, texture_views);
            }
        }

        let mut temp = vec![];

        for entry in self.entries.iter() {
            let resource = match &entry.resource {
                BindGroupResourceBinding::Sampler(sampler) => {
                    BindingResourceTemp::Sampler(sampler.wgpu_sampler().clone())
                }
                BindGroupResourceBinding::TextureView(binding) => {
                    let texture = render_context.get_resource(&binding.texture);
                    BindingResourceTemp::TextureView(
                        texture
                            .resource
                            .create_view(&binding.texture_view_info.get_texture_view_desc()),
                    )
                }
                BindGroupResourceBinding::Buffer(buffer_ref) => BindingResourceTemp::Buffer {
                    buffer: render_context.get_resource(&buffer_ref.buffer),
                    size: buffer_ref.size,
                    offset: buffer_ref.offset,
                },
                BindGroupResourceBinding::TextureViewArray(_) => {
                    let mut temp_texture_views = vec![];

                    let texture_views = resources.get(&entry.binding).unwrap();

                    for texture_view in texture_views {
                        temp_texture_views.push(texture_view);
                    }

                    BindingResourceTemp::TextureViewArray(temp_texture_views)
                }
            };

            temp.push((entry.binding, resource));
        }

        let bind_graoup = render_context
            .render_device
            .wgpu_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label.as_deref(),
                layout: &self.layout,
                entries: &temp
                    .iter()
                    .map(|(binding, resource)| wgpu::BindGroupEntry {
                        binding: *binding,
                        resource: resource.get_resource_binding(),
                    })
                    .collect::<Vec<_>>(),
            });

        bind_graoup
    }
}
