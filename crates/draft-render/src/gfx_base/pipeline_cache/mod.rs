use super::{ComputePipeline, Pipeline, RenderPipeline};

pub type CachedPipelineId = usize;

pub trait GetPipelineContainer {
    fn get_pipeline_container(&self) -> PipelineContainer;
}

pub struct PipelineContainer(Vec<Option<Pipeline>>);

impl PipelineContainer {
    pub fn new(value: Vec<Option<Pipeline>>) -> Self {
        Self(value)
    }

    pub fn get_render_pipeline(&self, id: CachedPipelineId) -> Option<&RenderPipeline> {
        self.0[id]
            .as_ref()
            .and_then(|pipelie| pipelie.get_render_pipeline())
    }

    pub fn get_compute_pipeline(&self, id: CachedPipelineId) -> Option<&ComputePipeline> {
        self.0[id]
            .as_ref()
            .and_then(|pipelie| pipelie.get_compute_pipeline())
    }
}
