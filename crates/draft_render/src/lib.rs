pub mod error;
pub mod frame_graph;
pub mod processor;
pub mod render_pipeline;
pub mod render_world;

use draft_graphics::RenderServer;
use draft_window::SystemWindowManager;

use crate::{
    processor::{CORE_2D, ProcessorContianer},
    render_pipeline::{RenderPipeline, RenderPipelineContainer, RenderPipelineRunContext},
    render_world::RenderWorld,
};

pub use error::FrameworkError;

pub trait IWorld: 'static {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    pub render_world: &'a mut RenderWorld,
    pub processor_contianer: &'a mut ProcessorContianer,
}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub render_pipeline_container: RenderPipelineContainer,
    pub render_world: RenderWorld,
    pub processor_contianer: ProcessorContianer,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_world: RenderWorld::new(&render_server.device),
            render_server,
            system_window_manager,
            render_pipeline_container: RenderPipelineContainer::default(),
            processor_contianer: ProcessorContianer::default(),
        }
    }

    pub fn initialize(&mut self) {
        self.render_pipeline_container
            .insert(CORE_2D, RenderPipeline::default());
    }

    pub fn render<W: IWorld>(&mut self, world: &W) {
        self.render_world
            .prepare_windows(&self.render_server, &self.system_window_manager);

        let mut context = RenderContext {
            render_world: &mut self.render_world,
            processor_contianer: &mut self.processor_contianer,
        };

        world.render(&mut context);

        let mut context = RenderPipelineRunContext {};

        if let Some(pipeline) = self.render_pipeline_container.get(CORE_2D) {
            pipeline.run(&mut context);
        }

        self.render_world
            .clear_windows(&self.render_server, &self.system_window_manager);
    }
}
