use crate::{
    frame_graph::{RenderPassCommand, RenderPassContext},
    gfx_base::GpuRenderPipeline,
};

pub struct SetRenderPipelineParameter {
    pub pipeline: GpuRenderPipeline,
}

impl RenderPassCommand for SetRenderPipelineParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_render_pipeline(&self.pipeline);
    }
}
