use draft_render::{
    MaterialEffect, MaterialEffectLoader, RenderServer, RenderWorld, Texture, TextureLoader,
    frame_graph::{FrameGraph, RenderContext, TextureView, TransientResourceCache},
};
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

use crate::{
    renderer::{
        BatchContainer, FrameGraphContext, RenderDataBundleStorage, RenderPipelineContainer,
    },
    scene::{DrawContext, DynRenderObject, SceneContainer},
};

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

    pub fn prepare(&mut self, scene_container: &SceneContainer) -> RenderDataBundleStorage {
        let batch_container = BatchContainer::default();
        let cameras = vec![];
        let mut render_data_bundle_storage = RenderDataBundleStorage::new(cameras, batch_container);

        let mut draw_context: DrawContext<'_> = DrawContext {
            cameras: &mut render_data_bundle_storage.cameras,
            mesh_render_data_bundle_storage: render_data_bundle_storage
                .mesh_render_data_bundle_storage
                .as_mut(),
        };

        scene_container.draw(&mut draw_context);
        render_data_bundle_storage
    }

    pub fn render_frame(
        &mut self,
        render_data_bundle_storage: RenderDataBundleStorage,
        texture_view: &TextureView,
    ) {
        if render_data_bundle_storage.cameras.is_empty() {
            return;
        }

        let mut command_buffers = vec![];
        let mut frame_graph = FrameGraph::default();
        let mut frame_graph_context =
            FrameGraphContext::new(&render_data_bundle_storage, texture_view.clone());

        frame_graph_context.alloc_camera_buffer(&mut self.world);

        for (index, observer) in render_data_bundle_storage.cameras.iter().enumerate() {
            if let Some(pipeline) = self
                .render_pipeline_container
                .get_mut(&observer.pipeline_name)
            {
                frame_graph_context.set_camera(index);

                pipeline.run(&mut frame_graph, &mut self.world, &frame_graph_context);

                frame_graph.compile();

                let mut context = RenderContext::new(
                    &self.world.pipeline_cache,
                    &self.world.server.device,
                    &mut self.transient_resource_cache,
                );

                frame_graph.execute(&mut context);

                frame_graph.reset();

                let mut camera_command_buffers = context.finish();

                command_buffers.append(&mut camera_command_buffers);
            }
        }

        self.world.server.queue.submit(command_buffers);
    }

    pub fn render(&mut self, scene_container: &SceneContainer, texture_view: &TextureView) {
        let render_data_bundle_storage = self.prepare(scene_container);

        self.render_frame(render_data_bundle_storage, texture_view);
    }
}

pub fn initialize_renderer(
    server: RenderServer,
    resource_manager: &ResourceManager,
) -> WorldRenderer {
    WorldRenderer::new(server, resource_manager)
}
