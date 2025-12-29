use draft_graphics::{frame_graph::RenderPassBuilder, gfx_base::{GpuRenderPipeline, PipelineContainer}};
use fyrox_core::err;

use crate::MeshAllocator;

#[derive(Default)]
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

pub struct RenderPhaseContext<'a> {
    pub pipeline_container: &'a PipelineContainer,
    pub mesh_allocator: &'a MeshAllocator,
}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    state: DrawState,
}

impl<'a, 'b> TrackedRenderPassBuilder<'a, 'b> {
    pub fn new(render_pass_builder: RenderPassBuilder<'a, 'b>) -> Self {
        Self {
            render_pass_builder,
            state: DrawState::default(),
        }
    }

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
