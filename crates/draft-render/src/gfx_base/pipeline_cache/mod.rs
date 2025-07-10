use super::{ComputePipeline, Pipeline, RenderPipeline};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedPipelineId(usize);

impl Default for CachedPipelineId {
    fn default() -> Self {
        CachedPipelineId(usize::MAX)
    }
}

impl CachedPipelineId {
    pub const INVALID: CachedPipelineId = CachedPipelineId::new(usize::MAX);

    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn is_valid(&self) -> bool {
        *self != Self::INVALID
    }
}

pub trait GetPipelineContainer {
    fn get_pipeline_container(&self) -> PipelineContainer;
}

pub struct PipelineContainer(Vec<Option<Pipeline>>);

impl PipelineContainer {
    pub fn new(value: Vec<Option<Pipeline>>) -> Self {
        Self(value)
    }

    pub fn get_render_pipeline(&self, id: CachedPipelineId) -> Option<&RenderPipeline> {
        self.0[id.0]
            .as_ref()
            .and_then(|pipelie| pipelie.get_render_pipeline())
    }

    pub fn get_compute_pipeline(&self, id: CachedPipelineId) -> Option<&ComputePipeline> {
        self.0[id.0]
            .as_ref()
            .and_then(|pipelie| pipelie.get_compute_pipeline())
    }
}
