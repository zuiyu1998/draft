use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::{
    FrameContext, RenderWorld,
    frame_graph::{FrameGraph, TextureView},
};

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

pub trait Node: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
    );
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn empty() -> Self {
        RenderPipeline { nodes: vec![] }
    }

    pub fn push_node<T: Node>(&mut self, node: T) {
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
