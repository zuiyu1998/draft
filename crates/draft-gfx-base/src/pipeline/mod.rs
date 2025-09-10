mod container;

pub use container::*;

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

#[derive(Clone, Debug)]
pub struct GpuComputePipeline(wgpu::ComputePipeline);

impl GpuComputePipeline {
    pub fn wgpu(&self) -> &wgpu::ComputePipeline {
        &self.0
    }

    pub fn new(pipeline: wgpu::ComputePipeline) -> Self {
        GpuComputePipeline(pipeline)
    }
}

#[derive(Clone, Debug)]
pub enum GpuPipeline {
    RenderPipeline(GpuRenderPipeline),
    ComputePipeline(GpuComputePipeline),
}

impl GpuPipeline {
    pub fn get_render_pipeline(&self) -> Option<&GpuRenderPipeline> {
        match self {
            GpuPipeline::RenderPipeline(res) => Some(res),
            _ => None,
        }
    }

    pub fn get_compute_pipeline(&self) -> Option<&GpuComputePipeline> {
        match self {
            GpuPipeline::ComputePipeline(res) => Some(res),
            _ => None,
        }
    }
}
