use crate::frame_graph::FrameGraphContext;

use super::{TextureViewRead, TextureViewWrite, TransientResourceBinding};

#[derive(Clone)]
pub struct ColorAttachment {
    pub view: TextureViewWrite,
    pub resolve_target: Option<TextureViewRead>,
    pub ops: wgpu::Operations<wgpu::Color>,
}

#[derive(Clone)]
pub struct ColorAttachmentOwned {
    pub view: wgpu::TextureView,
    pub resolve_target: Option<wgpu::TextureView>,
    pub ops: wgpu::Operations<wgpu::Color>,
}

impl ColorAttachmentOwned {
    pub fn get_render_pass_color_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: &self.view,
            resolve_target: self.resolve_target.as_ref(),
            ops: self.ops,
        }
    }
}

impl TransientResourceBinding for ColorAttachment {
    type Resource = ColorAttachmentOwned;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource {
        let view = self.view.make_resource(frame_graph_context);

        if let Some(resolve_target) = &self.resolve_target {
            let resolve_target = resolve_target.make_resource(frame_graph_context);

            ColorAttachmentOwned {
                view,
                resolve_target: Some(resolve_target),
                ops: self.ops,
            }
        } else {
            ColorAttachmentOwned {
                view,
                resolve_target: None,
                ops: self.ops,
            }
        }
    }
}
