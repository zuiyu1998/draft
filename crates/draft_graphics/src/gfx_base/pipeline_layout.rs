use wgpu::{PipelineLayout as WgpuPipelineLayout, PushConstantRange};

use crate::gfx_base::GpuBindGroupLayout;

#[derive(Debug, Clone)]
pub struct GpuPipelineLayout(WgpuPipelineLayout);

impl GpuPipelineLayout {
    pub fn new(value: WgpuPipelineLayout) -> Self {
        GpuPipelineLayout(value)
    }

    pub(crate) fn get_wgpu_pipeline_layout(&self) -> &WgpuPipelineLayout {
        &self.0
    }
}

pub struct PipelineLayoutDescriptor {
    pub label: Option<String>,
    pub bind_group_layouts: Vec<GpuBindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

#[derive(Debug, Clone)]
pub struct PipelineLayout(GpuPipelineLayout);

impl PipelineLayout {
    pub fn new(value: GpuPipelineLayout) -> Self {
        PipelineLayout(value)
    }

    pub fn value(&self) -> &GpuPipelineLayout {
        &self.0
    }
}
