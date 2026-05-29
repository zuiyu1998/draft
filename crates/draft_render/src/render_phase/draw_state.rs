use crate::{frame_graph::RenderPassBuilder, render_world::CachePipelineId};

pub struct DrawState {
    pipeline_id: Option<CachePipelineId>,
}

impl DrawState {
    pub fn is_pipeline_id_set(&self) -> bool {
        self.pipeline_id.is_some()
    }

    pub fn set_pipeline_id(&mut self, pipeline_id: CachePipelineId) {
        self.pipeline_id = Some(pipeline_id);
    }

    pub fn reset(&mut self) {
        self.pipeline_id = None;
    }
}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    draw_state: DrawState,
}

impl<'a, 'b> TrackedRenderPassBuilder<'a, 'b> {
    pub fn new(render_pass_builder: RenderPassBuilder<'a, 'b>) -> Self {
        Self {
            render_pass_builder,
            draw_state: DrawState { pipeline_id: None },
        }
    }

    pub fn reset(&mut self) {
        self.draw_state.reset();
    }

    pub fn set_render_pipeline(&mut self, pipeline_id: CachePipelineId) {
        if self.draw_state.is_pipeline_id_set() {
            panic!("Render pipeline ID has already been set for this render pass builder.");
        }
        self.render_pass_builder.set_render_pipeline(pipeline_id);
        self.draw_state.set_pipeline_id(pipeline_id);
    }
}
