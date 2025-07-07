use std::mem::take;

use wgpu::CommandEncoder;

use crate::frame_graph::{
    ColorAttachment, ColorAttachmentRecord, DepthStencilAttachment, FrameGraphContext,
    RenderPassCommand, RenderPassCommandBuilder, RenderPassRecord, TransientResourceBinding,
};

use super::EncoderExecutor;

#[derive(Default)]
pub struct RenderPassCommandContainer {
    logic_render_passes: Vec<LogicRenderPass>,
    current_logic_render_pass: LogicRenderPass,
}

#[derive(Default)]
pub struct LogicRenderPass {
    render_pass_record: RenderPassRecord,
    commands: Vec<RenderPassCommand>,
    valid: bool,
}

impl RenderPassCommandContainer {
    pub fn is_valid(&self) -> bool {
        !self.logic_render_passes.is_empty()
    }

    pub fn finish(&mut self) {
        let logic_render_pass = take(&mut self.current_logic_render_pass);

        if logic_render_pass.valid {
            self.logic_render_passes.push(logic_render_pass);
        }
    }

    pub fn set_pass_name(&mut self, name: &str) {
        self.current_logic_render_pass.render_pass_record.label = Some(name.to_string().into());
        self.current_logic_render_pass.valid = true;
    }

    pub fn set_depth_stencil_attachment(
        &mut self,
        depth_stencil_attachment: DepthStencilAttachment,
    ) {
        self.current_logic_render_pass
            .render_pass_record
            .depth_stencil_attachment = Some(depth_stencil_attachment);

        self.current_logic_render_pass.valid = true;
    }

    pub fn add_out_color_attachment(&mut self, color_attachment: Option<ColorAttachment>) {
        self.current_logic_render_pass
            .render_pass_record
            .out_color_attachments
            .push(color_attachment);

        self.current_logic_render_pass.valid = true;
    }

    pub fn add_color_attachments(
        &mut self,
        mut color_attachments: Vec<Option<ColorAttachmentRecord>>,
    ) {
        self.current_logic_render_pass
            .render_pass_record
            .color_attachments
            .append(&mut color_attachments);

        self.current_logic_render_pass.valid = true;
    }

    pub fn add_color_attachment(&mut self, color_attachment: Option<ColorAttachmentRecord>) {
        self.current_logic_render_pass
            .render_pass_record
            .color_attachments
            .push(color_attachment);

        self.current_logic_render_pass.valid = true;
    }
}

impl RenderPassCommandBuilder for RenderPassCommandContainer {
    fn push_render_pass_command(&mut self, value: RenderPassCommand) {
        self.current_logic_render_pass.commands.push(value);
    }
}

impl EncoderExecutor for RenderPassCommandContainer {
    fn execute(
        &self,
        command_encoder: &mut CommandEncoder,
        frame_graph_context: &mut FrameGraphContext,
    ) {
        for logic_render_pass in self.logic_render_passes.iter() {
            let render_pass_owned = logic_render_pass
                .render_pass_record
                .make_resource(frame_graph_context);
            let render_pass_context =
                frame_graph_context.begin_render_pass(command_encoder, &render_pass_owned);

            render_pass_context.execute(&logic_render_pass.commands);
        }
    }
}
