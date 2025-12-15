use wgpu::{
    ColorTargetState, DepthStencilState, MultisampleState, PipelineCompilationOptions,
    PrimitiveState,
};

use crate::{
    VertexBufferLayout,
    gfx_base::{GpuShaderModule, PipelineLayout},
};

pub struct VertexState<'a> {
    pub module: GpuShaderModule,
    pub entry_point: Option<String>,
    pub buffers: Vec<VertexBufferLayout>,
    pub compilation_options: PipelineCompilationOptions<'a>,
}

pub struct FragmentState<'a> {
    pub module: GpuShaderModule,
    pub entry_point: Option<String>,
    pub targets: Vec<Option<ColorTargetState>>,
    pub compilation_options: PipelineCompilationOptions<'a>,
}

#[derive(Clone, Debug)]
pub struct GpuRenderPipeline(wgpu::RenderPipeline);

impl GpuRenderPipeline {
    pub fn wgpu(&self) -> &wgpu::RenderPipeline {
        &self.0
    }

    pub fn new(pipeline: wgpu::RenderPipeline) -> Self {
        GpuRenderPipeline(pipeline)
    }
}

pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<String>,
    pub layout: Option<PipelineLayout>,
    pub vertex: VertexState<'a>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState<'a>>,
}
