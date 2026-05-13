use draft_graphics::{DepthStencilState, MultisampleState, PrimitiveState, RenderPipeline};

pub struct GpuVertexState {}

pub struct GpuFragmentState {}

pub struct GpuRenderPipelineDescriptor {
    pub label: String,
    pub vertex: GpuVertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<GpuFragmentState>,
}

pub struct PipelineCache {}

impl PipelineCache {
    pub fn create_render_pipeline(
        &mut self,
        _desc: &GpuRenderPipelineDescriptor,
    ) -> RenderPipeline {
        // self.device.create_render_pipelie(&desc.create_desc())
        todo!()
    }
}
