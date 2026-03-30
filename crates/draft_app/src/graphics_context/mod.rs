#[cfg(feature = "winit")]
mod winit;

use draft_render::{
    FrameworkError,
    render_server::{RenderServer, RenderServerSetting},
    renderer::{World, WorldRenderer},
};
use draft_window::{SystemWindow, Window};

pub trait RenderServerConstructor {
    fn construct(
        &self,
        setting: &RenderServerSetting,
        window: Window,
    ) -> Result<(RenderServer, SystemWindow), FrameworkError>;
}

pub enum GraphicsContext {
    Initialized(InitializedGraphicsContext),
    Uninitialized(GraphicsContextParams),
}

impl GraphicsContext {
    pub fn render<W: World>(&mut self, world: &mut W) {
        if let GraphicsContext::Initialized(context) = self {
            world.render(&mut context.renderer.render_context());
            context.renderer.render();
        }
    }
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(GraphicsContextParams {
            render_server_setting: Default::default(),
            window: Default::default(),
        })
    }
}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,

    pub renderer: WorldRenderer,
}

#[derive(Debug, Clone)]
pub struct GraphicsContextParams {
    pub render_server_setting: RenderServerSetting,
    pub window: Window,
}
