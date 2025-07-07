use std::borrow::Cow;

use crate::frame_graph::FrameGraphContext;

use super::{
    ColorAttachment, ColorAttachmentRecord, DepthStencilAttachment, DepthStencilAttachmentOwned,
    TransientResourceBinding,
};

#[derive(Default)]
pub struct RenderPassRecord {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachmentRecord>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachment>,
    pub out_color_attachments: Vec<Option<ColorAttachment>>,
}

pub struct RenderPass {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachment>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachmentOwned>,
}

impl TransientResourceBinding for RenderPassRecord {
    type Resource = RenderPass;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource {
        let mut color_attachments = self.out_color_attachments.clone();

        for color_attachment in self.color_attachments.iter() {
            if color_attachment.is_none() {
                color_attachments.push(None);
            } else {
                color_attachments.push(Some(
                    color_attachment
                        .as_ref()
                        .unwrap()
                        .make_resource(frame_graph_context),
                ));
            }
        }

        let mut depth_stencil_attachment_owned = None;

        if let Some(depth_stencil_attachment) = &self.depth_stencil_attachment {
            depth_stencil_attachment_owned =
                Some(depth_stencil_attachment.make_resource(frame_graph_context));
        }

        RenderPass {
            label: self.label.clone(),
            color_attachments,
            depth_stencil_attachment: depth_stencil_attachment_owned,
        }
    }
}

impl RenderPass {
    pub fn create_render_pass(
        &self,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'static> {
        let depth_stencil_attachment =
            self.depth_stencil_attachment
                .as_ref()
                .map(|depth_stencil_attachment| {
                    depth_stencil_attachment.get_render_pass_depth_stencil_attachment()
                });

        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label.as_deref(),
            color_attachments: &self
                .color_attachments
                .iter()
                .map(|color_attachment| {
                    color_attachment
                        .as_ref()
                        .map(|color_attachment| color_attachment.get_render_pass_color_attachment())
                })
                .collect::<Vec<_>>(),
            depth_stencil_attachment,
            ..Default::default()
        });

        render_pass.forget_lifetime()
    }
}
