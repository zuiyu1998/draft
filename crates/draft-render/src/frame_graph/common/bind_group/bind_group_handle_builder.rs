use std::borrow::Cow;

use draft_gfx_base::GpuBindGroupLayout;

use crate::frame_graph::{
    BindGroupBufferHandleHelper, BindGroupEntryHandle, BindGroupHandle,
    BindGroupTextureViewHandleHelper, FrameGraph, IntoBindGroupResourceHandle,
};

pub struct BindGroupHandleBuilder<'a> {
    pub label: Option<Cow<'static, str>>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<BindGroupEntryHandle>,
    frame_graph: &'a mut FrameGraph,
}

impl<'a> BindGroupHandleBuilder<'a> {
    pub fn new(
        label: Option<Cow<'static, str>>,
        layout: GpuBindGroupLayout,
        frame_graph: &'a mut FrameGraph,
    ) -> Self {
        Self {
            label,
            layout,
            entries: vec![],
            frame_graph,
        }
    }

    pub fn frame_graph_mut(&mut self) -> &mut FrameGraph {
        self.frame_graph
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

    pub fn add_buffer<T: BindGroupBufferHandleHelper>(self, binding: u32, value: &T) -> Self {
        let handle = value
            .make_bind_group_buffer_handle(self.frame_graph)
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

    pub fn build(self) -> BindGroupHandle {
        BindGroupHandle {
            label: self.label,
            layout: self.layout,
            entries: self.entries,
        }
    }
}
