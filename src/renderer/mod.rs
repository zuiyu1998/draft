mod pipeline;

pub use pipeline::*;

use draft_render::{
    RenderServer, RenderWorld,
    frame_graph::{FrameGraph, RenderContext, TransientResourceCache},
};
use fyrox_resource::manager::ResourceManager;

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub pipeline: Pipeline,
    pub transient_resource_cache: TransientResourceCache,
}

impl WorldRenderer {
    pub fn new(server: RenderServer, resource_manager: &ResourceManager) -> Self {
        WorldRenderer {
            world: RenderWorld::new(server, resource_manager),
            pipeline: Pipeline::empty(),
            transient_resource_cache: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.world.update(dt);
    }

    pub fn render(&mut self, pipeline_context: &PipelineContext) {
        let mut frame_graph = FrameGraph::default();

        self.pipeline
            .run(&mut frame_graph, &mut self.world, pipeline_context);

        frame_graph.compile();

        let mut render_context = RenderContext::new(
            &self.world.server.device,
            &mut self.transient_resource_cache,
            &self.world.material_storage,
        );

        frame_graph.execute(&mut render_context);

        let command_buffers = render_context.finish();

        self.world.server.queue.wgpu_queue().submit(command_buffers);
    }
}
