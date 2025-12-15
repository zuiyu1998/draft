mod parameter;
mod pass_builder;
mod render_pass_builder;

pub use parameter::RenderPassExt;
pub use pass_builder::*;
pub use render_pass_builder::*;

use crate::frame_graph::{
    GraphRawResourceHandle, Handle, Pass, Ref, ResourceRead, ResourceWrite, TransientResource,
};

use super::{FrameGraph, ResourceMaterial};

pub struct PassNodeBuilder<'a> {
    pub(crate) graph: &'a mut FrameGraph,
    pub(crate) name: String,
    writes: Vec<GraphRawResourceHandle>,
    reads: Vec<GraphRawResourceHandle>,
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
    ) -> Ref<M::ResourceType, ResourceRead>;

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> Ref<M::ResourceType, ResourceWrite>;

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceRead>;

    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceWrite>;
}

impl<'a> PassNodeBuilderExt for PassNodeBuilder<'a> {
    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceWrite> {
        let index = resource_handle.raw.index;
        let desc = resource_handle.desc.clone();

        let resource_node = &mut self.graph.get_resource_node_mut(&index);
        resource_node.new_version();

        let new_raw = GraphRawResourceHandle {
            index,
            version: resource_node.version(),
        };

        self.writes.push(new_raw.clone());

        Ref::new(new_raw, desc)
    }

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceRead> {
        let raw = resource_handle.raw;
        let desc = resource_handle.desc.clone();

        if !self.reads.contains(&raw) {
            self.reads.push(raw.clone());
        }

        Ref::new(raw, desc)
    }
    fn read_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> Ref<M::ResourceType, ResourceRead> {
        let handle = material.imported(self.graph);
        self.read(handle)
    }

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> Ref<M::ResourceType, ResourceWrite> {
        let handle = material.imported(self.graph);
        self.write(handle)
    }
}

impl<'a> PassNodeBuilder<'a> {
    pub fn set_pass(&mut self, mut pass: Pass) {
        pass.label = Some(self.name.clone().into());
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
