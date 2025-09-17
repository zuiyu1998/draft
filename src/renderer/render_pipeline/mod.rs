use std::ops::{Deref, DerefMut};

use draft_render::{
    GeometryResource, MaterialResource, RenderWorld,
    frame_graph::{FrameGraph, TextureView},
};
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::renderer::{FrameContext, MeshInstanceData, RenderDataBundleStorage};

#[derive(Clone)]
pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instance_data: MeshInstanceData,
}

impl Batch {
    pub fn new(
        geometry: GeometryResource,
        material: MaterialResource,
        instance_data: MeshInstanceData,
    ) -> Self {
        Self {
            geometry,
            material,
            instance_data,
        }
    }
}

#[derive(Default)]
pub struct BatchContainer {
    pub batches: FxHashMap<u64, Batch>,
}

impl RenderDataBundleStorage for BatchContainer {
    fn push_mesh(
        &mut self,
        _geometry: GeometryResource,
        _material: MaterialResource,
        _sort_index: u64,
        _instance_data: MeshInstanceData,
    ) {
        todo!()
    }

    fn render_frame(&self, _render_world: &mut RenderWorld) {
        todo!()
    }
}

pub struct RenderPipelineContainer(FxHashMap<ImmutableString, RenderPipeline>);

impl Default for RenderPipelineContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelineContainer {
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn new() -> Self {
        RenderPipelineContainer::empty()
    }
}

impl Deref for RenderPipelineContainer {
    type Target = FxHashMap<ImmutableString, RenderPipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderPipelineContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct FrameGraphContext<'a> {
    pub frame_context: &'a FrameContext,
    pub camera: Option<usize>,
    pub texture_view: TextureView,
}

pub trait RenderPipelineNode: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
    );
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn RenderPipelineNode>>,
}

impl RenderPipeline {
    pub fn empty() -> Self {
        RenderPipeline { nodes: vec![] }
    }

    pub fn push_node<T: RenderPipelineNode>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, frame_graph_context);
        }
    }
}
