use std::{borrow::Cow, num::NonZero};

use wgpu::BindGroupLayout;

use crate::{
    frame_graph::{
        BindGroupBinding, BindGroupBufferBinding, BindGroupResourceBinding,
        BindGroupResourceBindingHelper, BindGroupTextureViewBinding, Handle,
        IntoBindGroupResourceBinding, PassNodeBuilder, TextureViewInfo, TransientBuffer,
        TransientTexture,
    },
    resource::Sampler,
};

use super::BindGroupEntryBinding;

#[derive(Clone)]
pub struct BindGroupHandle {
    pub label: Option<Cow<'static, str>>,
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntryHandle>,
}

impl BindGroupHandle {
    pub fn make_binding(&self, pass_node_builder: &mut PassNodeBuilder) -> BindGroupBinding {
        let entries = self
            .entries
            .iter()
            .map(|entry| entry.get_binding(pass_node_builder))
            .collect::<Vec<_>>();

        BindGroupBinding {
            label: self.label.clone(),
            layout: self.layout.clone(),
            entries,
        }
    }
}

#[derive(Clone)]
pub struct BindGroupEntryHandle {
    pub binding: u32,
    pub resource: BindGroupResourceHandle,
}

impl BindGroupEntryHandle {
    pub fn get_binding(&self, pass_node_builder: &mut PassNodeBuilder) -> BindGroupEntryBinding {
        let resource = self
            .resource
            .make_bind_group_resource_binding(pass_node_builder);

        BindGroupEntryBinding {
            binding: self.binding,
            resource,
        }
    }
}

#[derive(Clone)]
pub enum BindGroupResourceHandle {
    Buffer(BindGroupBufferHandle),
    Sampler(Sampler),
    TextureView(BindGroupTextureViewHandle),
    TextureViewArray(Vec<BindGroupTextureViewHandle>),
}

impl BindGroupResourceBindingHelper for BindGroupResourceHandle {
    fn make_bind_group_resource_binding(
        &self,
        pass_node_builder: &mut PassNodeBuilder,
    ) -> BindGroupResourceBinding {
        match &self {
            BindGroupResourceHandle::Buffer(handle) => {
                let buffer = pass_node_builder.read(handle.buffer.clone());
                BindGroupBufferBinding {
                    buffer,
                    size: handle.size,
                    offset: handle.offset,
                }
                .into_binding()
            }
            BindGroupResourceHandle::Sampler(info) => {
                BindGroupResourceBinding::Sampler(info.clone())
            }
            BindGroupResourceHandle::TextureView(handle) => {
                let texture = pass_node_builder.read(handle.texture.clone());

                BindGroupResourceBinding::TextureView(BindGroupTextureViewBinding {
                    texture,
                    texture_view_info: handle.texture_view_info.clone(),
                })
            }
            BindGroupResourceHandle::TextureViewArray(handles) => {
                let mut target = vec![];
                for handle in handles.iter() {
                    let texture = pass_node_builder.read(handle.texture.clone());
                    target.push(BindGroupTextureViewBinding {
                        texture,
                        texture_view_info: handle.texture_view_info.clone(),
                    });
                }

                BindGroupResourceBinding::TextureViewArray(target)
            }
        }
    }
}

#[derive(Clone)]
pub struct BindGroupBufferHandle {
    pub buffer: Handle<TransientBuffer>,
    pub size: Option<NonZero<u64>>,
    pub offset: u64,
}

#[derive(Clone)]
pub struct BindGroupTextureViewHandle {
    pub texture: Handle<TransientTexture>,
    pub texture_view_info: TextureViewInfo,
}

pub trait IntoBindGroupResourceHandle {
    fn into_binding(self) -> BindGroupResourceHandle;
}

impl IntoBindGroupResourceHandle for &BindGroupResourceHandle {
    fn into_binding(self) -> BindGroupResourceHandle {
        self.clone()
    }
}

impl IntoBindGroupResourceHandle for BindGroupResourceHandle {
    fn into_binding(self) -> BindGroupResourceHandle {
        self
    }
}

impl IntoBindGroupResourceHandle for &[BindGroupTextureViewHandle] {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::TextureViewArray(self.to_vec())
    }
}

impl IntoBindGroupResourceHandle for BindGroupTextureViewHandle {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::TextureView(self)
    }
}

impl IntoBindGroupResourceHandle for &BindGroupTextureViewHandle {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::TextureView(self.clone())
    }
}

impl IntoBindGroupResourceHandle for Handle<TransientBuffer> {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Buffer(BindGroupBufferHandle {
            buffer: self,
            size: None,
            offset: 0,
        })
    }
}

impl IntoBindGroupResourceHandle for (&Handle<TransientBuffer>, u64, Option<NonZero<u64>>) {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Buffer(BindGroupBufferHandle {
            buffer: self.0.clone(),
            size: self.2,
            offset: self.1,
        })
    }
}

impl IntoBindGroupResourceHandle for &Handle<TransientBuffer> {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Buffer(BindGroupBufferHandle {
            buffer: self.clone(),
            size: None,
            offset: 0,
        })
    }
}

impl IntoBindGroupResourceHandle for BindGroupBufferHandle {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Buffer(self)
    }
}

impl IntoBindGroupResourceHandle for Sampler {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Sampler(self)
    }
}

impl IntoBindGroupResourceHandle for &Sampler {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::Sampler(self.clone())
    }
}

impl IntoBindGroupResourceHandle for (&Handle<TransientTexture>, &TextureViewInfo) {
    fn into_binding(self) -> BindGroupResourceHandle {
        BindGroupResourceHandle::TextureView(BindGroupTextureViewHandle {
            texture: self.0.clone(),
            texture_view_info: self.1.clone(),
        })
    }
}
