use crate::frame_graph::FrameGraphContext;

use super::{TextureViewWrite, TransientResourceBinding};

#[derive(Clone)]
pub struct DepthStencilAttachment {
    pub view: TextureViewWrite,
    pub depth_ops: Option<wgpu::Operations<f32>>,
    pub stencil_ops: Option<wgpu::Operations<u32>>,
}

impl TransientResourceBinding for DepthStencilAttachment {
    type Resource = DepthStencilAttachmentOwned;

    fn make_resource(&self, frame_graph_context: &FrameGraphContext<'_>) -> Self::Resource {
        let view = self.view.make_resource(frame_graph_context);

        DepthStencilAttachmentOwned {
            view,
            depth_ops: self.depth_ops,
            stencil_ops: self.stencil_ops,
        }
    }
}

pub struct DepthStencilAttachmentOwned {
    pub view: wgpu::TextureView,
    pub depth_ops: Option<wgpu::Operations<f32>>,
    pub stencil_ops: Option<wgpu::Operations<u32>>,
}

impl DepthStencilAttachmentOwned {
    pub fn get_render_pass_depth_stencil_attachment(
        &self,
    ) -> wgpu::RenderPassDepthStencilAttachment {
        wgpu::RenderPassDepthStencilAttachment {
            view: &self.view,
            depth_ops: self.depth_ops,
            stencil_ops: self.stencil_ops,
        }
    }
}
