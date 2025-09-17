mod frame_context;
mod material;
mod observer;
mod render_pipeline;

pub use frame_context::*;
pub use material::*;
pub use observer::*;
pub use render_pipeline::*;

use draft_render::{
    MaterialEffect, MaterialEffectLoader, RenderServer, RenderWorld, Texture, TextureLoader,
    frame_graph::{FrameGraph, RenderContext, TextureView, TransientResourceCache},
};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

use crate::scene::{DrawContext, SceneContainer};

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub render_pipeline_container: RenderPipelineContainer,
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
            render_pipeline_container: Default::default(),
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
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event
                && let Some(texture) = resource.try_cast::<Texture>()
            {
                self.world.update_texture(&texture);
            }
        }
    }

    fn update_material_effect(&mut self) {
        while let Ok(event) = self.material_effect_event_receiver.try_recv()
            && let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event
        {
            if let Some(material_effect) = resource.try_cast::<MaterialEffect>() {
                let container = self.world.material_effect_container.clone();
                if let Some(material_effect) = material_effect.data_ref().as_loaded_ref() {
                    container.register_material_effect(material_effect.clone());
                }
            }
        }
    }

    pub fn prepare(
        &mut self,
        observers_collection: &mut ObserversCollection,
        scene_container: &SceneContainer,
    ) -> Option<FrameContext> {
        let mut batch_container = BatchContainer::default();

        let mut draw_context: DrawContext<'_> = DrawContext {
            observers_collection,
            render_data_bundle_storage: &mut batch_container,
        };

        draw_context.collect_render_data(scene_container);

        observers_collection
            .prepare(&self.world)
            .map(|camera_uniforms| FrameContext::new(camera_uniforms, batch_container))
    }

    pub fn render_frame(
        &mut self,
        frame_context: &FrameContext,
        observers_collection: &ObserversCollection,
        texture_view: &TextureView,
    ) {
        let mut command_buffers = vec![];

        for (index, observer) in observers_collection.cameras.iter().enumerate() {
            if let Some(pipeline) = self
                .render_pipeline_container
                .get_mut(&observer.pipeline_name)
            {
                let mut frame_graph = FrameGraph::default();

                let frame_context = FrameGraphContext {
                    camera: Some(index),
                    frame_context,
                    texture_view: texture_view.clone(),
                };

                pipeline.run(&mut frame_graph, &mut self.world, &frame_context);

                frame_graph.compile();

                let mut context = RenderContext::new(
                    &self.world.pipeline_cache,
                    &self.world.server.device,
                    &mut self.transient_resource_cache,
                );

                frame_graph.execute(&mut context);

                let mut camera_command_buffers = context.finish();

                command_buffers.append(&mut camera_command_buffers);
            }
        }

        self.world.server.queue.submit(command_buffers);
    }

    pub fn render(&mut self, scene_container: &SceneContainer, texture_view: &TextureView) {
        let mut observers_collection = ObserversCollection::default();

        if let Some(frame_context) = self.prepare(&mut observers_collection, scene_container) {
            self.render_frame(&frame_context, &observers_collection, texture_view);
        }
    }
}

pub fn initialize_renderer(
    server: RenderServer,
    resource_manager: &ResourceManager,
) -> WorldRenderer {
    WorldRenderer::new(server, resource_manager)
}
