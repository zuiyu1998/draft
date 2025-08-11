use crate::{
    frame_graph::PassContext,
    gfx_base::{Color, Operations},
};

use super::{TextureView, TextureViewInfoRead, TextureViewInfoWrite};

#[derive(Clone)]
pub struct ColorAttachmentInfo {
    pub view: TextureViewInfoWrite,
    pub resolve_target: Option<TextureViewInfoRead>,
    pub ops: Operations<Color>,
}

#[derive(Clone)]
pub struct ColorAttachment {
    pub view: TextureView,
    pub resolve_target: Option<TextureView>,
    pub ops: Operations<Color>,
}

impl ColorAttachment {
    pub fn get_render_pass_color_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: self.view.get_gpu_texture_view(),
            resolve_target: self
                .resolve_target
                .as_ref()
                .map(|view| view.get_gpu_texture_view()),
            ops: self.ops,
        }
    }

    pub fn new(context: &PassContext<'_>, info: &ColorAttachmentInfo) -> Self {
        let view = TextureView::from_info(context, &info.view);
        if let Some(resolve_target) = &info.resolve_target {
            let resolve_target = TextureView::from_info(context, resolve_target);

            ColorAttachment {
                view,
                resolve_target: Some(resolve_target),
                ops: info.ops,
            }
        } else {
            ColorAttachment {
                view,
                resolve_target: None,
                ops: info.ops,
            }
        }
    }
}
