use crate::{
    frame_graph::FrameGraphContext,
    gfx_base::{Color, Operations, RawTextureView},
};

use super::{TextureViewRead, TextureViewWrite, TransientResourceBinding};

#[derive(Clone)]
pub struct ColorAttachmentRecord {
    pub view: TextureViewWrite,
    pub resolve_target: Option<TextureViewRead>,
    pub ops: Operations<Color>,
}

#[derive(Clone)]
pub struct ColorAttachment {
    pub view: RawTextureView,
    pub resolve_target: Option<RawTextureView>,
    pub ops: Operations<Color>,
}

impl ColorAttachment {
    pub fn get_render_pass_color_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: &self.view,
            resolve_target: self.resolve_target.as_ref(),
            ops: self.ops,
        }
    }
}

impl TransientResourceBinding for ColorAttachmentRecord {
    type Resource = ColorAttachment;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource {
        let view = self.view.make_resource(frame_graph_context);

        if let Some(resolve_target) = &self.resolve_target {
            let resolve_target = resolve_target.make_resource(frame_graph_context);

            ColorAttachment {
                view,
                resolve_target: Some(resolve_target),
                ops: self.ops,
            }
        } else {
            ColorAttachment {
                view,
                resolve_target: None,
                ops: self.ops,
            }
        }
    }
}
