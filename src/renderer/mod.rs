mod material;
mod render_data_bundle;
mod render_pipeline;
mod world_renderer;

pub use material::*;
pub use render_data_bundle::*;
pub use render_pipeline::*;
pub use world_renderer::*;

pub enum GraphicsContext {
    Initialized(Box<InitializedGraphicsContext>),
    Uninitialized(GraphicsContextParams),
}

impl Default for GraphicsContext {
    fn default() -> Self {
        Self::Uninitialized(Default::default())
    }
}

#[derive(Default)]
pub struct GraphicsContextParams {}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,
    pub renderer: WorldRenderer,
}
