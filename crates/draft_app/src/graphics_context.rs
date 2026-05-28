use draft_graphics::RenderServer;
use draft_render::{IWorld, RenderOptions, WorldRenderer};
use draft_window::SystemWindow;

pub type RenderServerConstructor =
    Box<dyn Fn(&GraphicsContextParams, SystemWindow) -> RenderServer>;

pub enum GraphicsContext {
    /// Fully initialized graphics context. See [`InitializedGraphicsContext`] docs for more info.
    Initialized(InitializedGraphicsContext),

    /// Uninitialized (suspended) graphics context. See [`GraphicsContextParams`] docs for more info.
    Uninitialized(GraphicsContextParams),
}

impl GraphicsContext {
    pub fn update(&mut self) {
        //todo
    }

    pub fn render<W: IWorld>(&mut self, world: &W) {
        if let GraphicsContext::Initialized(initialized_graphics_context) = self {
            initialized_graphics_context.renderer.render(world);
        }
    }
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(GraphicsContextParams {
            options: RenderOptions::default(),
        })
    }
}

#[derive(Clone)]
pub struct GraphicsContextParams {
   pub options: RenderOptions,
}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,
    pub renderer: WorldRenderer,
}
