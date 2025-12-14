use crate::{
    GeometryInstanceData, MaterialResource, PipelineCache, RenderDataBundle, RenderServer,
};
use draft_geometry::GeometryResource;
use draft_window::Window;
use fyrox_resource::manager::ResourceManager;

pub struct WorldRenderer {
    _render_server: RenderServer,
    pipeline_cache: PipelineCache,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, _resource_manager: &ResourceManager) -> Self {
        Self {
            pipeline_cache: PipelineCache::new(render_server.device.clone()),
            _render_server: render_server,
        }
    }

    pub fn update(&mut self) {
        self.pipeline_cache.process_queue();
    }

    fn prepare_frame<W: World>(&mut self, world: &W) -> RenderFrame {
        let mut render_data_bundle = RenderDataBundle::empty();

        let mut context = RenderContext {
            render_data_bundle: &mut render_data_bundle,
        };

        world.prepare(&mut context);

        let frame = Frame {
            _render_data_bundle: render_data_bundle,
        };

        frame.prepare()
    }

    fn render_frame(&mut self, _frame: RenderFrame) {}

    pub fn render<W: World>(&mut self, world: &W) {
        let frame = self.prepare_frame(world);
        self.render_frame(frame);
    }
}

pub struct Frame {
    _render_data_bundle: RenderDataBundle,
}

impl Frame {
    pub fn prepare(&self) -> RenderFrame {
        RenderFrame {}
    }
}

pub struct RenderFrame {}

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
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(Default::default())
    }
}
