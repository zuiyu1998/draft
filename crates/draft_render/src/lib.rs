pub mod frame_graph;
pub mod render_pipeline;

use draft_graphics::RenderServer;
use draft_window::SystemWindowManager;

use crate::render_pipeline::{RenderPipeline, RenderPipelineContainer, RenderPipelineRunContext};

pub const CORE_2D: &'static str = "core_2d";

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub render_pipeline_container: RenderPipelineContainer,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_server,
            system_window_manager,
            render_pipeline_container: RenderPipelineContainer::default(),
        }
    }

    pub fn initialize(&mut self) {
        self.render_pipeline_container
            .insert(CORE_2D, RenderPipeline::default());
    }

    pub fn render(&mut self) {
        let mut context = RenderPipelineRunContext {};

        if let Some(pipeline) = self.render_pipeline_container.get(CORE_2D) {
            pipeline.run(&mut context);
        }
    }
}
