mod render_frame;
mod render_pipeline;
mod render_resource;
mod renderer;

pub mod error;

use std::sync::Arc;

use draft_shader::Shader;
use fyrox_resource::Resource;
pub use render_frame::*;
pub use render_pipeline::*;
pub use render_resource::*;
pub use renderer::*;

use draft_graphics::{
    frame_graph::{
        FrameGraph, Handle, ResourceMaterial, TransientBuffer, TransientBufferDescriptor,
    },
    gfx_base::Buffer,
};
use draft_window::Window;

pub struct BufferMeta {
    key: String,
    value: Buffer,
}

impl ResourceMaterial for BufferMeta {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<Self::ResourceType> {
        frame_graph.import(
            &self.key,
            Arc::new(TransientBuffer {
                resource: self.value.value.clone(),
                desc: TransientBufferDescriptor::from_buffer_desc(&self.value.desc),
            }),
        )
    }
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
