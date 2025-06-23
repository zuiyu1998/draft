use std::borrow::Cow;

use wgpu::BindGroupLayout;

use crate::frame_graph::{
    BindGroupEntryHandle, BindGroupHandle, BindGroupResourceHandleHelper,
    BindGroupTextureViewHandleHelper, FrameGraph, IntoBindGroupResourceHandle,
};

pub struct BindGroupHandleBuilder<'a> {
    pub label: Option<Cow<'static, str>>,
    pub layout: BindGroupLayout,
    pub entries: Vec<BindGroupEntryHandle>,
    frame_graph: &'a mut FrameGraph,
}

impl<'a> BindGroupHandleBuilder<'a> {
    pub fn new(
        label: Option<Cow<'static, str>>,
        layout: BindGroupLayout,
        frame_graph: &'a mut FrameGraph,
    ) -> Self {
        Self {
            label,
            layout,
            entries: vec![],
            frame_graph,
        }
    }
    pub fn add_texture_view<T: BindGroupTextureViewHandleHelper>(
        self,
        binding: u32,
        value: &T,
    ) -> Self {
        let handle = value
            .make_bind_group_texture_view_handle(self.frame_graph)
            .into_binding();
        self.add_handle(binding, handle)
    }

    pub fn add_handle<T: IntoBindGroupResourceHandle>(mut self, binding: u32, handle: T) -> Self {
        self.entries.push(BindGroupEntryHandle {
            binding,
            resource: handle.into_binding(),
        });

        self
    }

    pub fn add_helper<T: BindGroupResourceHandleHelper>(self, binding: u32, value: &T) -> Self {
        let handle = value.make_bind_group_resource_handle(self.frame_graph);
        self.add_handle(binding, handle)
    }

    pub fn build(self) -> BindGroupHandle {
        BindGroupHandle {
            label: self.label,
            layout: self.layout,
            entries: self.entries,
        }
    }
}
