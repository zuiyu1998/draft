use draft_graphics::{frame_graph::RenderPassBuilder, gfx_base::GpuRenderPipeline};
use fyrox_core::err;

pub struct DrawState {
    pipeline: Option<GpuRenderPipeline>,
}

impl DrawState {
    pub fn is_pipeline_set(&self) -> bool {
        self.pipeline.is_some()
    }

    pub fn set_render_pipeline(&mut self, pipeline: &GpuRenderPipeline) {
        self.pipeline = Some(pipeline.clone())
    }
}

pub struct RenderPhaseContext {}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    state: DrawState,
}

impl TrackedRenderPassBuilder<'_, '_> {
    pub fn set_render_pipeline(&mut self, pipeline: &GpuRenderPipeline) {
        if self.state.is_pipeline_set() {
            err!("There are multiple rendering pipeline for the same drawcall.");
            return;
        }

        self.render_pass_builder.set_render_pipeline(pipeline);
        self.state.set_render_pipeline(pipeline);
    }
}

pub trait RenderPhase: 'static {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, context: &RenderPhaseContext);
}
