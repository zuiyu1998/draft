mod render_frame;
mod render_phase;
mod render_pipeline;
mod render_resource;
mod renderer;

pub mod error;

pub use render_frame::*;
pub use render_phase::*;
pub use render_pipeline::*;
pub use render_resource::*;
pub use renderer::*;
pub use error::FrameworkError;

use draft_shader::Shader;
use fyrox_resource::Resource;

use draft_window::Window;

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
    pub fn update(&mut self, dt: f32) {
        if let GraphicsContext::Initialized(graphics_context) = self {
            graphics_context.renderer.update(dt);
        }
    }

    pub fn render<W: World>(&mut self, world: &W) {
        if let GraphicsContext::Initialized(graphics_context) = self {
            graphics_context.renderer.render(world);
        }
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        if let GraphicsContext::Initialized(graphics_context) = self {
            graphics_context.renderer.set_shader(shader);
        }
    }
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(Default::default())
    }
}
