mod processor;
mod render_world;

pub use processor::*;
pub use render_world::*;

use crate::{
    BufferAllocator, MaterialEffectCache, RenderFrame, RenderPipeline, RenderPipelineContext,
    RenderPipelineExt, RenderPipelineManager, RenderServer, RenderWindow, RenderWindows,
    error::FrameworkError,
};
use draft_graphics::{
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
    gfx_base::{TextureView, TextureViewDescriptor},
};
use draft_material::MaterialEffectResource;
use draft_shader::Shader;
use draft_window::SystemWindowManager;
use fyrox_resource::{Resource, manager::ResourceManager};
use tracing::error;

pub struct WorldRenderer {
    render_server: RenderServer,
    render_pipeline_manager: RenderPipelineManager,
    system_window_manager: SystemWindowManager,
    buffer_allocator: BufferAllocator,
    transient_resource_cache: TransientResourceCache,
    material_effect_cache: MaterialEffectCache,
    render_world: RenderWorld,
}

impl RenderPipelineExt for WorldRenderer {
    fn insert_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        self.render_pipeline_manager.insert_pipeline(name, pipeline);
    }

    fn pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.render_pipeline_manager.pipeline(name)
    }

    fn pipeline_mut(&mut self, name: &str) -> Option<&mut RenderPipeline> {
        self.render_pipeline_manager.pipeline_mut(name)
    }
}

impl WorldRenderer {
    pub fn new(
        render_server: RenderServer,
        system_window_manager: SystemWindowManager,
        resource_manager: &ResourceManager,
    ) -> Self {
        Self {
            material_effect_cache: MaterialEffectCache::new(
                render_server.device.clone(),
                resource_manager,
            ),
            buffer_allocator: BufferAllocator::new(render_server.device.clone()),
            render_world: RenderWorld::new(&render_server, resource_manager),
            render_server,
            render_pipeline_manager: RenderPipelineManager::default(),
            system_window_manager,
            transient_resource_cache: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.material_effect_cache.update(dt);
        self.buffer_allocator.update(dt);
        self.render_world.update(dt);
    }

    pub fn set_material_effect(&mut self, material_effect: MaterialEffectResource) {
        self.material_effect_cache
            .set_material_effect(material_effect);
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        self.render_world.set_shader(shader);
    }

    fn prepare_render_windows(&self) -> Result<RenderWindows, FrameworkError> {
        let mut windows = RenderWindows::default();

        for (index, window) in self.system_window_manager.get_ref().iter().enumerate() {
            if let Some(current_texture) = window.get_current_texture() {
                let current_texture = current_texture?;

                let texure_view = TextureView::new(
                    current_texture.create_view(&TextureViewDescriptor::default()),
                );

                windows.insert(
                    index,
                    RenderWindow {
                        surface_texture: current_texture,
                        surface_texture_view: texure_view,
                    },
                );
            }
        }

        let primary = self.system_window_manager.get_primary();

        windows.set_primary(primary.index() as usize);

        Ok(windows)
    }

    fn clear_render_windows(&self, windows: RenderWindows) {
        for window in windows.into_iter() {
            window.surface_texture.present();
        }
    }

    fn prepare_frame<W: World>(&mut self, world: &W) -> Result<RenderFrame, FrameworkError> {
        self.buffer_allocator.unset();

        let windows = self.prepare_render_windows()?;

        world.prepare(&mut self.render_world);

        let render_data_bundle = self.render_world.prepare_render_data_bundle();

        Ok(RenderFrame {
            windows,
            render_data_bundle,
        })
    }

    fn render_frame(&mut self, frame: RenderFrame) {
        self.render_pipeline_manager.update();

        let context = RenderPipelineContext {
            pipeline_container: &frame.render_data_bundle.pipeline_container,
            mesh_allocator: &self.render_world.mesh_cache.allocator,
            render_device: &self.render_server.device,
        };
        let mut frame_graph = FrameGraph::default();

        if let Some(pipeline) = self.render_pipeline_manager.pipeline_mut("core_2d") {
            pipeline.run(&mut frame_graph, &frame, &context);
        }

        frame_graph.compile();

        let mut frame_graph_context = FrameGraphContext::new(
            frame.render_data_bundle.pipeline_container,
            &self.render_server.device,
            &mut self.transient_resource_cache,
        );

        frame_graph.execute(&mut frame_graph_context);

        let command_buffers = frame_graph_context.finish();

        self.render_server.queue.submit(command_buffers);

        self.clear_render_windows(frame.windows);
    }

    pub fn render<W: World>(&mut self, world: &W) {
        match self.prepare_frame(world) {
            Ok(frame) => {
                self.render_frame(frame);
            }
            Err(e) => {
                error!("Render error: {}", e);
            }
        };
    }
}
