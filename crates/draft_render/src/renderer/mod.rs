use crate::{
    Frame, GeometryInstanceData, PipelineCache, RenderDataBundle, RenderFrame, RenderFrameContext,
    RenderPipelineManager, RenderServer, RenderWindows, SpecializedMeshPipeline,
    error::FrameworkError,
};
use draft_geometry::{GeometryResource, GeometryVertexBufferLayouts};
use draft_graphics::frame_graph::FrameGraph;
use draft_material::MaterialResource;
use draft_window::{SystemWindowManager, Window};
use fyrox_resource::manager::ResourceManager;
use tracing::error;

pub struct WorldRenderer {
    _render_server: RenderServer,
    pipeline_cache: PipelineCache,
    specialized_mesh_pipeline: SpecializedMeshPipeline,
    render_pipeline_manager: RenderPipelineManager,
    layouts: GeometryVertexBufferLayouts,
    system_window_manager: SystemWindowManager,
}

impl WorldRenderer {
    pub fn new(
        render_server: RenderServer,
        system_window_manager: SystemWindowManager,
        _resource_manager: &ResourceManager,
    ) -> Self {
        Self {
            pipeline_cache: PipelineCache::new(render_server.device.clone()),
            _render_server: render_server,
            specialized_mesh_pipeline: Default::default(),
            render_pipeline_manager: Default::default(),
            layouts: Default::default(),
            system_window_manager,
        }
    }

    pub fn update(&mut self) {
        self.pipeline_cache.process_queue();
    }

    pub fn prepare_render_windows(&self) -> Result<RenderWindows, FrameworkError> {
        let mut windows = RenderWindows::default();

        for (index, window) in self.system_window_manager.get_ref().iter().enumerate() {
            if let Some(current_texture) = window.get_current_texture() {
                let current_texture = current_texture?;

                todo!()
            }
        }

        Ok(windows)
    }

    fn prepare_frame<W: World>(&mut self, world: &W) -> Result<RenderFrame, FrameworkError> {
        let mut render_data_bundle = RenderDataBundle::empty();

        let mut context = RenderContext {
            render_data_bundle: &mut render_data_bundle,
        };

        world.prepare(&mut context);

        let frame = Frame {
            render_data_bundle: render_data_bundle,
            windows: RenderWindows::default(),
        };

        frame.prepare(
            &mut self.specialized_mesh_pipeline,
            &mut self.pipeline_cache,
            &mut self.layouts,
        )
    }

    fn render_frame(&mut self, frame: RenderFrame) {
        self.render_pipeline_manager.update();

        let mut frame_graph = FrameGraph::default();
        let context = RenderFrameContext { frame: &frame };

        if let Some(pipeline) = self.render_pipeline_manager.pipeline_mut("2d") {
            pipeline.run(&mut frame_graph, &context);
        }
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
            .push(geometry, material, instance);
    }
}

pub trait World {
    fn prepare(&self, context: &mut RenderContext);
}

pub struct InitializedGraphicsContext {
    pub renderer: WorldRenderer,
    pub params: GraphicsContextParams,
}

impl InitializedGraphicsContext {
    pub fn new(renderer: WorldRenderer, params: GraphicsContextParams) -> Self {
        Self { renderer, params }
    }
}

#[derive(Default, Clone)]
pub struct GraphicsContextParams {
    pub window: Window,
}

pub enum GraphicsContext {
    Initialized(InitializedGraphicsContext),
    Uninitialized(GraphicsContextParams),
}

impl GraphicsContext {
    pub fn update(&mut self) {
        if let GraphicsContext::Initialized(graphics_context) = self {
            graphics_context.renderer.update();
        }
    }

    pub fn render<W: World>(&mut self, world: &W) {
        if let GraphicsContext::Initialized(graphics_context) = self {
            graphics_context.renderer.render(world);
        }
    }
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(Default::default())
    }
}
