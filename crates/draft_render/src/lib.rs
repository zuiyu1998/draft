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
