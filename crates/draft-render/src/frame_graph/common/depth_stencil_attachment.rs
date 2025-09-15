use crate::frame_graph::{PassContext, TextureView, TextureViewInfoWrite};
use draft_gfx_base::WgpuRenderPassDepthStencilAttachment;

#[derive(Clone)]
pub struct DepthStencilAttachmentInfo {
    pub view: TextureViewInfoWrite,
    pub depth_ops: Option<wgpu::Operations<f32>>,
    pub stencil_ops: Option<wgpu::Operations<u32>>,
}

pub struct DepthStencilAttachment {
    pub view: TextureView,
    pub depth_ops: Option<wgpu::Operations<f32>>,
    pub stencil_ops: Option<wgpu::Operations<u32>>,
}

impl DepthStencilAttachment {
    pub fn get_render_pass_depth_stencil_attachment<'a>(
        &'a self,
    ) -> WgpuRenderPassDepthStencilAttachment<'a> {
        WgpuRenderPassDepthStencilAttachment {
            view: self.view.get_gpu_texture_view().get_texture_view(),
            depth_ops: self.depth_ops,
            stencil_ops: self.stencil_ops,
        }
    }

    pub fn new(context: &PassContext<'_>, info: &DepthStencilAttachmentInfo) -> Self {
        let view = TextureView::from_info(context, &info.view);

        DepthStencilAttachment {
            view,
            depth_ops: info.depth_ops,
            stencil_ops: info.stencil_ops,
        }
    }
}
