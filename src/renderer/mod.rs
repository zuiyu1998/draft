mod observer;
mod pipeline;

pub use observer::*;
pub use pipeline::*;

use draft_render::{
    MaterialEffect, MaterialEffectLoader, RenderPhasesContainer, RenderServer, RenderWorld,
    Texture, TextureLoader,
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub pipeline_container: PipelineContainer,
    pub transient_resource_cache: TransientResourceCache,
    texture_event_receiver: Receiver<ResourceEvent>,
    material_effect_event_receiver: Receiver<ResourceEvent>,
}

impl WorldRenderer {
    pub fn new(server: RenderServer, resource_manager: &ResourceManager) -> Self {
        let (texture_event_sender, texture_event_receiver) = std::sync::mpsc::channel();
        let (material_effect_event_sender, material_effect_event_receiver) =
            std::sync::mpsc::channel();

        resource_manager
            .state()
            .event_broadcaster
            .add(texture_event_sender);

        resource_manager
            .state()
            .event_broadcaster
            .add(material_effect_event_sender);

        resource_manager.add_loader(TextureLoader::default());
        resource_manager.add_loader(MaterialEffectLoader);

        WorldRenderer {
            world: RenderWorld::new(server),
            pipeline_container: Default::default(),
            transient_resource_cache: Default::default(),
            texture_event_receiver,
            material_effect_event_receiver,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_material_effect();
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

    fn update_material_effect(&mut self) {
        while let Ok(event) = self.material_effect_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(material_effect) = resource.try_cast::<MaterialEffect>() {
                    let container = self.world.material_effect_container.clone();
                    if let Some(material_effect) = material_effect.data_ref().as_loaded_ref() {
                        container.register_material_effect(material_effect.clone());
                    }
                }
            }
        }
    }

    pub fn prepare(
        &mut self,
        pipeline_context: &PipelineContext,
        render_phases_container: &mut RenderPhasesContainer,
    ) {
        let mut context = PhaseContext {
            world: &mut self.world,
            render_phases_container,
        };

        let _ = pipeline_context.batch.extra(&mut context);

        //todo
    }

    pub fn render_frame(
        &mut self,
        pipeline_context: &PipelineContext,
        render_phases_container: &RenderPhasesContainer,
    ) {
        //camera uniform
        let observer_data = ObserversData::new(&pipeline_context.observers_collection, &self.world);

        let mut command_buffers = vec![];

        for (index, observer) in pipeline_context
            .observers_collection
            .cameras
            .iter()
            .enumerate()
        {
            if let Some(pipeline) = self.pipeline_container.get_mut(&observer.pipeline_name) {
                let mut frame_graph = FrameGraph::default();

                let frame_context = FrameContext {
                    context: pipeline_context,
                    render_phases_container,
                    observer_data: &observer_data,
                    camera: Some(index),
                };

                pipeline.run(&mut frame_graph, &mut self.world, &frame_context);

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

        self.world.server.queue.submit(command_buffers);
    }

    pub fn render(&mut self, pipeline_context: &PipelineContext) {
        let mut phases_container = RenderPhasesContainer::default();

        self.prepare(pipeline_context, &mut phases_container);

        self.render_frame(pipeline_context, &phases_container);
    }
}

pub fn initialize_renderer(
    server: RenderServer,
    resource_manager: &ResourceManager,
) -> WorldRenderer {
    WorldRenderer::new(server, resource_manager)
}
