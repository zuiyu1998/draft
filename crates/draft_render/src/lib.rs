
pub mod render_resource;
pub mod render_server;


use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {}

pub struct WorldRenderer {}

pub enum GraphicsContext {
    Initialized(InitializedGraphicsContext),
    Uninitialized(GraphicsContextParams),
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(GraphicsContextParams {})
    }
}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,

    pub renderer: WorldRenderer,
}

pub struct GraphicsContextParams {}
