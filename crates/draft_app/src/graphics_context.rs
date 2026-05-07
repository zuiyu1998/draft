use draft_render::{RenderServer, WorldRenderer};
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
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(GraphicsContextParams {})
    }
}

#[derive(Clone)]
pub struct GraphicsContextParams {}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,
    pub renderer: WorldRenderer,
}
