mod observer;
mod pipeline;

pub use observer::*;
pub use pipeline::*;

use draft_render::{
    BufferAllocator, RenderPhasesContainer, RenderServer, RenderWorld, Texture, TextureLoader,
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub pipeline_container: PipelineContainer,
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
            pipeline_container: Default::default(),
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
        render_phases_container: &mut RenderPhasesContainer,
    ) {
        let mut buffer_allocator = BufferAllocator::default();
        let mut context = PhaseContext {
            world: &mut self.world,
            render_phases_container,
            buffer_allocator: &mut buffer_allocator,
        };

        let _ = pipeline_context.batch.extra(&mut context);

        //todo
    }

    pub fn render_frame(
        &mut self,
        pipeline_context: &PipelineContext,
        render_phases_container: &RenderPhasesContainer,
        observers: &ObserversCollection,
    ) {
        let mut command_buffers = vec![];

        for observer in observers.cameras.iter() {
            if let Some(pipeline) = self.pipeline_container.get_mut(&observer.pipeline_name) {
                let mut frame_graph = FrameGraph::default();

                pipeline.run(
                    &mut frame_graph,
                    &mut self.world,
                    pipeline_context,
                    render_phases_container,
                );

                frame_graph.compile();
                let mut context = FrameGraphContext::new(
                    &self.world.pipeline_cache,
                    &self.world.server.device,
                    &mut self.transient_resource_cache,
                );

                frame_graph.execute(&mut context);

                let mut frame_command_buffers = context.finish();

                command_buffers.append(&mut frame_command_buffers);
            }
        }

        self.world.server.queue.wgpu_queue().submit(command_buffers);
    }

    pub fn render(&mut self, pipeline_context: &PipelineContext, observers: &ObserversCollection) {
        let mut phases_container = RenderPhasesContainer::default();

        self.prepare(pipeline_context, &mut phases_container);

        self.render_frame(pipeline_context, &phases_container, observers);
    }
}
