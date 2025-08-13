use crate::{GpuBindGroupLayout, RawPipelineLayout};

pub struct PipelineLayoutDescriptor<'a> {
    pub bind_group_layouts: Vec<&'a GpuBindGroupLayout>,
}

#[derive(Clone)]
pub struct GpuPipelineLayout(RawPipelineLayout);

impl GpuPipelineLayout {
    pub fn new(pipeline_layout: RawPipelineLayout) -> Self {
        GpuPipelineLayout(pipeline_layout)
    }

    pub fn get_pipeline_layout(&self) -> &RawPipelineLayout {
        &self.0
    }
}
