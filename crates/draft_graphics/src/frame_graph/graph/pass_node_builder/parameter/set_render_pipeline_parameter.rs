use crate::{
    frame_graph::{RenderPassCommand, RenderPassContext},
    gfx_base::CachedPipelineId,
};

pub struct SetRenderPipelineParameter {
    pub id: CachedPipelineId,
}

impl RenderPassCommand for SetRenderPipelineParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_render_pipeline(self.id);
    }
}
