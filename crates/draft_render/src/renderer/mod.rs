use crate::{
    BufferAllocator, Frame, GeometryInstanceData, PipelineCache, RenderDataBundle, RenderFrame,
    RenderFrameContext, RenderPipeline, RenderPipelineExt, RenderPipelineManager, RenderServer,
    RenderWindow, RenderWindows, SpecializedMeshPipeline, error::FrameworkError,
};
use draft_geometry::{GeometryResource, GeometryVertexBufferLayouts};
use draft_graphics::{
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
    gfx_base::{GetPipelineContainer, TextureView, TextureViewDescriptor},
};
use draft_material::MaterialResource;
use draft_shader::Shader;
use draft_window::SystemWindowManager;
use fyrox_resource::{Resource, manager::ResourceManager};
use tracing::error;

pub struct WorldRenderer {
    render_server: RenderServer,
    pipeline_cache: PipelineCache,
    specialized_mesh_pipeline: SpecializedMeshPipeline,
    render_pipeline_manager: RenderPipelineManager,
    layouts: GeometryVertexBufferLayouts,
    system_window_manager: SystemWindowManager,
    buffer_allocator: BufferAllocator,
    transient_resource_cache: TransientResourceCache,
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
            pipeline_cache: PipelineCache::new(render_server.device.clone(), resource_manager),
            buffer_allocator: BufferAllocator::new(render_server.device.clone()),
            render_server,
            specialized_mesh_pipeline: Default::default(),
            render_pipeline_manager: RenderPipelineManager::default(),
            layouts: Default::default(),
            system_window_manager,
            transient_resource_cache: Default::default(),
        }
    }

    pub fn update(&mut self) {
        self.pipeline_cache.update();
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        self.pipeline_cache.set_shader(shader);
    }

    pub fn prepare_render_windows(&self) -> Result<RenderWindows, FrameworkError> {
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
        let mut render_data_bundle = RenderDataBundle::empty();

        let mut context = RenderContext {
            render_data_bundle: &mut render_data_bundle,
            layouts: &mut self.layouts,
        };

        world.prepare(&mut context);

        let windows = self.prepare_render_windows()?;

        let frame = Frame {
            render_data_bundle: render_data_bundle,
            windows,
        };

        frame.prepare(
            &mut self.specialized_mesh_pipeline,
            &mut self.pipeline_cache,
            &mut self.layouts,
            &mut self.buffer_allocator,
            &self.render_server.queue,
        )
    }

    fn render_frame(&mut self, frame: RenderFrame) {
        self.render_pipeline_manager.update();

        let pipeline_container = self.pipeline_cache.get_pipeline_container();

        let context = RenderFrameContext {
            frame: &frame,
            pipeline_container: &pipeline_container,
        };
        let mut frame_graph = FrameGraph::default();

        if let Some(pipeline) = self.render_pipeline_manager.pipeline_mut("core_2d") {
            pipeline.run(&mut frame_graph, &context);
        }

        frame_graph.compile();

        let mut frame_graph_context = FrameGraphContext::new(
            pipeline_container,
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

pub struct RenderContext<'a> {
    render_data_bundle: &'a mut RenderDataBundle,
    layouts: &'a mut GeometryVertexBufferLayouts,
}

impl RenderContext<'_> {
    pub fn push(
        &mut self,
        geometry: GeometryResource,
        material: MaterialResource,
        instance: GeometryInstanceData,
    ) {
        self.render_data_bundle
            .mesh
            .push(geometry, material, instance, self.layouts);
    }
}

pub trait World {
    fn prepare(&self, context: &mut RenderContext);
}
