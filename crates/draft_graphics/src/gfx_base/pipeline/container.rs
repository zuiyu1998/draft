use super::{GpuComputePipeline, GpuPipeline, GpuRenderPipeline};

pub type CachedPipelineId = usize;

pub trait GetPipelineContainer {
    fn get_pipeline_container(&self) -> PipelineContainer;
}

pub struct PipelineContainer(Vec<Option<GpuPipeline>>);

impl PipelineContainer {
    pub fn new(value: Vec<Option<GpuPipeline>>) -> Self {
        Self(value)
    }

    pub fn get_render_pipeline(&self, id: CachedPipelineId) -> Option<&GpuRenderPipeline> {
        self.0[id]
            .as_ref()
            .and_then(|pipelie| pipelie.get_render_pipeline())
    }

    pub fn get_compute_pipeline(&self, id: CachedPipelineId) -> Option<&GpuComputePipeline> {
        self.0[id]
            .as_ref()
            .and_then(|pipelie| pipelie.get_compute_pipeline())
    }
}
