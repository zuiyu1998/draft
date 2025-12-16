use std::collections::HashMap;

use draft_graphics::frame_graph::FrameGraph;

use crate::RenderFrame;

pub struct RenderFrameContext<'a> {
    pub frame: &'a RenderFrame,
}

pub trait Node: 'static + Sync + Send {
    fn update(&mut self);

    fn run(&self, frame_graph: &mut FrameGraph, context: &RenderFrameContext);
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn update(&mut self) {
        for node in self.nodes.iter_mut() {
            node.update();
        }
    }

    pub fn run(&self, frame_graph: &mut FrameGraph, context: &RenderFrameContext) {
        for node in self.nodes.iter() {
            node.run(frame_graph, context);
        }
    }
}

pub struct RenderPipelineManager {
    data: HashMap<String, RenderPipeline>,
}

impl Default for RenderPipelineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelineManager {
    pub fn new() -> Self {
        RenderPipelineManager {
            data: Default::default(),
        }
    }

    pub fn update(&mut self) {
        for pipeline in self.data.values_mut() {
            pipeline.update();
        }
    }

    pub fn insert(&mut self, name: &str, pipeline: RenderPipeline) {
        self.data.insert(name.to_string(), pipeline);
    }

    pub fn pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.data.get(name)
    }

    pub fn pipeline_mut(&mut self, name: &str) -> Option<&mut RenderPipeline> {
        self.data.get_mut(name)
    }
}
