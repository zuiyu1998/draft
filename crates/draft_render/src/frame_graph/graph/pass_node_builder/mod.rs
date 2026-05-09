mod parameter;
mod pass_builder;
mod render_pass_builder;

pub use parameter::RenderPassExt;
pub use pass_builder::*;
pub use render_pass_builder::*;

use crate::frame_graph::{
    FrameGraph, Pass, RawResourceHandle, ResourceHandle, ResourceMaterial, ResourceRead,
    ResourceRef, ResourceWrite, TransientResource, TransientTextureView,
    TransientTextureViewDescriptor, TransientTextureViewHandle,
};

pub struct PassNodeBuilder<'a> {
    pub(crate) graph: &'a mut FrameGraph,
    pub(crate) name: String,
    writes: Vec<RawResourceHandle>,
    reads: Vec<RawResourceHandle>,
    pass: Option<Pass>,
}

impl Drop for PassNodeBuilder<'_> {
    fn drop(&mut self) {
        let pass_node = self.graph.pass_node(&self.name);
        pass_node.writes = self.writes.clone();
        pass_node.reads = self.reads.clone();
        pass_node.pass = self.pass.take();
    }
}

pub trait PassNodeBuilderExt {
    fn read_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceRead>;

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceWrite>;

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceRead>;

    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceWrite>;

    fn read_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView;

    fn write_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView;
}

impl<'a> PassNodeBuilderExt for PassNodeBuilder<'a> {
    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceWrite> {
        let index = resource_handle.raw.index;
        let desc = resource_handle.desc.clone();

        let resource_node = &mut self.graph.get_resource_node_mut(&index);
        resource_node.new_version();

        let new_raw = RawResourceHandle {
            index,
            version: resource_node.version(),
        };

        self.writes.push(new_raw.clone());

        ResourceRef::new(new_raw, desc)
    }

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceRead> {
        let raw = resource_handle.raw;
        let desc = resource_handle.desc.clone();

        if !self.reads.contains(&raw) {
            self.reads.push(raw.clone());
        }

        ResourceRef::new(raw, desc)
    }
    fn read_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceRead> {
        let handle = material.imported(self.graph);
        self.read(handle)
    }

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceWrite> {
        let handle = material.imported(self.graph);
        self.write(handle)
    }

    fn read_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView {
        match texture_handle {
            TransientTextureViewHandle::Descriptor(handle) => {
                let texture = self.read(handle.texture.clone());

                TransientTextureView::Read(TransientTextureViewDescriptor {
                    texture,
                    desc: handle.desc.clone(),
                })
            }
            TransientTextureViewHandle::TextureView(texture_view) => {
                TransientTextureView::TextureView(texture_view.clone())
            }
        }
    }

    fn write_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView {
        match texture_handle {
            TransientTextureViewHandle::Descriptor(handle) => {
                let texture = self.write(handle.texture.clone());

                TransientTextureView::Write(TransientTextureViewDescriptor {
                    texture,
                    desc: handle.desc.clone(),
                })
            }
            TransientTextureViewHandle::TextureView(texture_view) => {
                TransientTextureView::TextureView(texture_view.clone())
            }
        }
    }
}

impl<'a> PassNodeBuilder<'a> {
    pub(crate) fn set_pass(&mut self, mut pass: Pass) {
        pass.label = Some(self.name.clone());
        self.pass = Some(pass);
    }

    pub fn new(name: &str, graph: &'a mut FrameGraph) -> Self {
        Self {
            graph,
            name: name.to_string(),
            writes: vec![],
            reads: vec![],
            pass: None,
        }
    }
}
