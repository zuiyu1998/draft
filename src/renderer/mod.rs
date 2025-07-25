mod pipeline;

pub use pipeline::*;

use draft_render::{
    PhasesContainer, RenderServer, RenderWorld, Texture, TextureLoader,
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub pipeline: Pipeline,
    pub transient_resource_cache: TransientResourceCache,
    texture_event_receiver: Receiver<ResourceEvent>,
}

impl WorldRenderer {
    pub fn new(server: RenderServer, resource_manager: &ResourceManager) -> Self {
        let (texture_event_sender, texture_event_receiver) = std::sync::mpsc::channel();

        resource_manager
            .state()
            .event_broadcaster
            .add(texture_event_sender);

        resource_manager.add_loader(TextureLoader::default());

        WorldRenderer {
            world: RenderWorld::new(server),
            pipeline: Pipeline::empty(),
            transient_resource_cache: Default::default(),
            texture_event_receiver,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_texture();
        self.world.update(dt);
    }

    fn update_texture(&mut self) {
        while let Ok(event) = self.texture_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(texture) = resource.try_cast::<Texture>() {
                    self.world.update_texture(&texture);
                }
            }
        }
    }

    pub fn prepare(
        &mut self,
        pipeline_context: &PipelineContext,
        phases_container: &mut PhasesContainer,
    ) {
        let _ = pipeline_context
            .batch
            .extra(&mut self.world, phases_container);

        //todo
    }

    pub fn render_frame(
        &mut self,
        pipeline_context: &PipelineContext,
        phases_container: &PhasesContainer,
    ) {
        let mut frame_graph = FrameGraph::default();

        self.pipeline.run(
            &mut frame_graph,
            &mut self.world,
            pipeline_context,
            phases_container,
        );

        frame_graph.compile();

        let mut render_context = FrameGraphContext::new(
            &self.world.server.device,
            &mut self.transient_resource_cache,
            &self.world.pipeline_cache,
        );

        frame_graph.execute(&mut render_context);

        let command_buffers = render_context.finish();

        self.world.server.queue.wgpu_queue().submit(command_buffers);
    }

    pub fn render(&mut self, pipeline_context: &PipelineContext) {
        let mut phases_container = PhasesContainer::default();

        self.prepare(pipeline_context, &mut phases_container);

        self.render_frame(pipeline_context, &phases_container);
    }
}
