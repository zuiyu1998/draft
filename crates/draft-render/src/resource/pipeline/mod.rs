#[derive(Clone)]
pub struct RenderPipeline(wgpu::RenderPipeline);

impl RenderPipeline {
    pub fn wgpu(&self) -> &wgpu::RenderPipeline {
        &self.0
    }

    pub fn new(pipeline: wgpu::RenderPipeline) -> Self {
        RenderPipeline(pipeline)
    }
}

#[derive(Clone)]
pub struct ComputePipeline(wgpu::ComputePipeline);

impl ComputePipeline {
    pub fn wgpu(&self) -> &wgpu::ComputePipeline {
        &self.0
    }

    pub fn new(pipeline: wgpu::ComputePipeline) -> Self {
        ComputePipeline(pipeline)
    }
}

#[derive(Clone)]
pub enum Pipeline {
    RenderPipeline(RenderPipeline),
    ComputePipeline(ComputePipeline),
}

impl Pipeline {
    pub fn get_render_pipeline(&self) -> Option<&RenderPipeline> {
        match self {
            Pipeline::RenderPipeline(res) => Some(res),
            _ => None,
        }
    }

    pub fn get_compute_pipeline(&self) -> Option<&ComputePipeline> {
        match self {
            Pipeline::ComputePipeline(res) => Some(res),
            _ => None,
        }
    }
}
