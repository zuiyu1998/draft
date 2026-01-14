use std::collections::HashMap;

use draft_graphics::{
    frame_graph::FrameGraph,
    gfx_base::{PipelineContainer, RenderDevice},
};

use crate::{BufferAllocator, MeshAllocator, RenderFrame};

pub struct RenderPipelineContext<'a> {
    pub pipeline_container: &'a PipelineContainer,
    pub mesh_allocator: &'a MeshAllocator,
    pub buffer_allocator: &'a BufferAllocator,
    pub render_device: &'a RenderDevice,
}

pub trait Node: 'static + Sync + Send {
    fn update(&mut self) {}

    fn run(
        &self,
        frame_graph: &mut FrameGraph,
        render_frame: &RenderFrame,
        context: &RenderPipelineContext,
    );
}

#[derive(Default)]
pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn push_node(&mut self, value: impl Node) {
        self.nodes.push(Box::new(value));
    }

    pub fn update(&mut self) {
        for node in self.nodes.iter_mut() {
            node.update();
        }
    }

    pub fn run(
        &self,
        frame_graph: &mut FrameGraph,
        render_frame: &RenderFrame,
        context: &RenderPipelineContext,
    ) {
        for node in self.nodes.iter() {
            node.run(frame_graph, render_frame, context);
        }
    }
}

pub trait RenderPipelineExt {
    fn insert_pipeline(&mut self, name: &str, pipeline: RenderPipeline);

    fn pipeline(&self, name: &str) -> Option<&RenderPipeline>;

    fn pipeline_mut(&mut self, name: &str) -> Option<&mut RenderPipeline>;
}

pub struct RenderPipelineManager {
    data: HashMap<String, RenderPipeline>,
}

impl Default for RenderPipelineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelineExt for RenderPipelineManager {
    fn insert_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        self.data.insert(name.to_string(), pipeline);
    }

    fn pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.data.get(name)
    }

    fn pipeline_mut(&mut self, name: &str) -> Option<&mut RenderPipeline> {
        self.data.get_mut(name)
    }
}

impl RenderPipelineManager {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn empty() -> Self {
        RenderPipelineManager {
            data: Default::default(),
        }
    }

    pub fn update(&mut self) {
        for pipeline in self.data.values_mut() {
            pipeline.update();
        }
    }
}
