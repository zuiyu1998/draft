use crate::{GeometryInstanceData, MaterialResource, RenderDataBundle, RenderServer};
use draft_geometry::GeometryResource;
use draft_window::Window;
use fyrox_resource::manager::ResourceManager;

pub struct WorldRenderer {
    _render_server: RenderServer,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, _resource_manager: &ResourceManager) -> Self {
        Self {
            _render_server: render_server,
        }
    }

    pub fn update(&mut self) {}

    fn prepare_frame<W: World>(&mut self, world: &W) -> Frame {
        let mut render_data_bundle = RenderDataBundle::empty();

        let mut context = RenderContext {
            render_data_bundle: &mut render_data_bundle,
        };

        world.prepare(&mut context);

        Frame {
            _render_data_bundle: render_data_bundle,
        }
    }

    fn render_frame(&mut self, _frame: Frame) {}

    pub fn render<W: World>(&mut self, world: &W) {
        let frame = self.prepare_frame(world);
        self.render_frame(frame);
    }
}

pub struct Frame {
    _render_data_bundle: RenderDataBundle,
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

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(Default::default())
    }
}
