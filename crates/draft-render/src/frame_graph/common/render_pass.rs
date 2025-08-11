use std::borrow::Cow;

use crate::frame_graph::PassContext;

use super::{
    ColorAttachment, ColorAttachmentInfo, DepthStencilAttachment, DepthStencilAttachmentInfo,
};

#[derive(Default)]
pub struct RenderPassInfo {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachmentInfo>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachmentInfo>,
    pub out_color_attachments: Vec<Option<ColorAttachment>>,
}

pub struct RenderPassDescriptor {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachment>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachment>,
}

impl RenderPassDescriptor {
    pub fn new(context: &PassContext<'_>, info: &RenderPassInfo) -> Self {
        let mut color_attachments = info.out_color_attachments.clone();

        for color_attachment in info.color_attachments.iter() {
            if color_attachment.is_none() {
                color_attachments.push(None);
            } else {
                color_attachments.push(Some(ColorAttachment::new(
                    context,
                    color_attachment.as_ref().unwrap(),
                )));
            }
        }

        let mut depth_stencil_attachment = None;

        if let Some(depth_stencil_attachment_info) = &info.depth_stencil_attachment {
            depth_stencil_attachment = Some(DepthStencilAttachment::new(
                context,
                depth_stencil_attachment_info,
            ));
        }

        RenderPassDescriptor {
            label: info.label.clone(),
            color_attachments,
            depth_stencil_attachment,
        }
    }
}

pub struct GpuRenderPass(wgpu::RenderPass<'static>);

impl GpuRenderPass {
    pub fn get_render_pass(&self) -> &wgpu::RenderPass<'static> {
        &self.0
    }

    pub fn get_render_pass_mut(&mut self) -> &mut wgpu::RenderPass<'static> {
        &mut self.0
    }
}

impl GpuRenderPass {
    pub fn new(command_encoder: &mut wgpu::CommandEncoder, desc: &RenderPassDescriptor) -> Self {
        let depth_stencil_attachment =
            desc.depth_stencil_attachment
                .as_ref()
                .map(|depth_stencil_attachment| {
                    depth_stencil_attachment.get_render_pass_depth_stencil_attachment()
                });

        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: desc.label.as_deref(),
            color_attachments: &desc
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

        GpuRenderPass(render_pass.forget_lifetime())
    }
}
