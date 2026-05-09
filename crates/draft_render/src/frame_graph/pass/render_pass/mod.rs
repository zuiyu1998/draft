mod context;
mod gpu_render_pass;

pub use context::*;
pub use gpu_render_pass::*;

use wgpu::{Color, Operations};

use crate::frame_graph::{PassCommand, PassContext, TransientTextureView};

pub struct TransientRenderPassColorAttachment {
    pub view: TransientTextureView,
    pub depth_slice: Option<u32>,
    pub resolve_target: Option<TransientTextureView>,
    pub ops: Operations<Color>,
}

pub struct TransientRenderPassDepthStencilAttachment {
    pub view: TransientTextureView,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

#[derive(Default)]
pub struct TransientRenderPassDescriptor {
    pub label: Option<String>,
    pub color_attachments: Vec<Option<TransientRenderPassColorAttachment>>,
    pub depth_stencil_attachment: Option<TransientRenderPassDepthStencilAttachment>,
}

pub trait RenderPassCommand: Sync + Send + 'static {
    fn execute(&self, render_pass_context: &mut RenderPassContext);
}

#[derive(Default)]
pub struct RenderPass {
    desc: TransientRenderPassDescriptor,
    pub(crate) commands: Vec<Box<dyn RenderPassCommand>>,
}

impl RenderPass {
    pub fn set_pass_name(&mut self, name: &str) {
        self.desc.label = Some(name.to_string());
    }

    pub fn add_color_attachment(
        &mut self,
        color_attachment: Option<TransientRenderPassColorAttachment>,
    ) {
        self.desc.color_attachments.push(color_attachment);
    }
}

impl PassCommand for RenderPass {
    fn execute(&self, context: &mut PassContext) {
        let desc = context.create_render_pass_descriptor(&self.desc);
        let render_pass = GpuRenderPass::begin_render_pass(&mut context.command_encoder, &desc);
        let mut render_pass_context = RenderPassContext::new(render_pass, context);

        for command in self.commands.iter() {
            command.execute(&mut render_pass_context);
        }
    }
}
