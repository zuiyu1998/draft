mod builder;
mod context;

pub use builder::*;
pub use context::*;

use crate::frame_graph::{
    ColorAttachment, ColorAttachmentInfo, DepthStencilAttachmentInfo, GpuRenderPass, PassCommand,
    PassContext, RenderPassDescriptor, RenderPassInfo,
};

#[derive(Default)]
pub struct RenderPass {
    info: RenderPassInfo,
    commands: Vec<Box<dyn RenderPassCommand>>,
}

impl RenderPass {
    pub fn set_pass_name(&mut self, name: &str) {
        self.info.label = Some(name.to_string().into());
    }

    pub fn set_depth_stencil_attachment(
        &mut self,
        depth_stencil_attachment: DepthStencilAttachmentInfo,
    ) {
        self.info.depth_stencil_attachment = Some(depth_stencil_attachment);
    }

    pub fn add_out_color_attachment(&mut self, color_attachment: Option<ColorAttachment>) {
        self.info.out_color_attachments.push(color_attachment);
    }

    pub fn add_color_attachments(
        &mut self,
        mut color_attachments: Vec<Option<ColorAttachmentInfo>>,
    ) {
        self.info.color_attachments.append(&mut color_attachments);
    }

    pub fn add_color_attachment(&mut self, color_attachment: Option<ColorAttachmentInfo>) {
        self.info.color_attachments.push(color_attachment);
    }
}

impl RenderPassCommandBuilder for RenderPass {
    fn push_render_pass_command<T: RenderPassCommand>(&mut self, value: T) {
        self.commands.push(Box::new(value));
    }
}

impl PassCommand for RenderPass {
    fn execute(&self, context: &mut PassContext) {
        let desc = RenderPassDescriptor::new(context, &self.info);
        let render_pass = GpuRenderPass::new(&mut context.command_encoder, &desc);
        let mut render_pass_context = RenderPassContext::new(render_pass, context);

        for command in self.commands.iter() {
            command.execute(&mut render_pass_context);
        }
    }
}
