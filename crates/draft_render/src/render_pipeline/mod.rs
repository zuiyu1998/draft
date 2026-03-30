mod core_2d;

use std::collections::HashMap;

pub use core_2d::*;
use draft_window::SystemWindowManager;

use crate::{render_resource::RenderWorld, render_server::RenderServer};

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
        let mut manager = Self::empty();
        manager.add_pipeline(CORE_2D, create_core_2d_pipeline());

        manager
    }

    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        self.data.insert(name.to_string(), pipeline);
    }

    pub fn get_pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.data.get(name)
    }
}

pub struct RenderPipelineContext<'a> {
    system_window_manager: &'a SystemWindowManager,
    render_world: &'a RenderWorld,
    render_server: &'a RenderServer,
}

impl<'a> RenderPipelineContext<'a> {
    pub fn new(
        system_window_manager: &'a SystemWindowManager,
        render_world: &'a RenderWorld,
        render_server: &'a RenderServer,
    ) -> Self {
        Self {
            system_window_manager,
            render_world,
            render_server,
        }
    }
}

pub trait Node {
    fn run(&self, _context: &mut RenderPipelineContext) {}
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_boxed_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    pub fn run(&self, context: &mut RenderPipelineContext) {
        for node in self.nodes.iter() {
            node.run(context);
        }
    }
}
