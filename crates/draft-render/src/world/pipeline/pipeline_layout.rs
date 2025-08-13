use draft_gfx_base::{GpuBindGroupLayout, GpuPipelineLayout, PipelineLayoutDescriptor};
use fxhash::FxHashMap;
use std::sync::Arc;

use crate::{
    FrameworkError,
    gfx_base::{BindGroupLayoutDescriptor, RawBindGroupLayout, RawPipelineLayout, RenderDevice},
};

#[derive(Default)]
pub struct PipelineLayoutCache {
    pipeline_layout_cache: FxHashMap<Vec<BindGroupLayoutDescriptor>, PipelineLayout>,
    bind_group_layout_cache: FxHashMap<BindGroupLayoutDescriptor, BindGroupLayout>,
}

impl PipelineLayoutCache {
    pub fn get_or_create_bind_group_layout(
        &mut self,
        device: &RenderDevice,
        desc: &BindGroupLayoutDescriptor,
    ) -> &BindGroupLayout {
        self.bind_group_layout_cache
            .entry(desc.clone())
            .or_insert_with(|| BindGroupLayout::new(device, desc));

        self.bind_group_layout_cache.get(desc).unwrap()
    }

    pub fn get_or_create_pipeline_layout(
        &mut self,
        device: &RenderDevice,
        descs: &[BindGroupLayoutDescriptor],
    ) -> Result<&PipelineLayout, FrameworkError> {
        let pipeline_layout_key = descs.to_vec();

        self.pipeline_layout_cache
            .entry(pipeline_layout_key)
            .or_insert_with(|| {
                PipelineLayout::new(device, descs, &mut self.bind_group_layout_cache)
            });

        todo!()
    }
}

#[derive(Clone)]
pub struct BindGroupLayout(Arc<GpuBindGroupLayout>);

impl BindGroupLayout {
    pub fn new(device: &RenderDevice, desc: &BindGroupLayoutDescriptor) -> Self {
        let bind_group_layout = device.create_bind_group_layout(desc);

        BindGroupLayout(Arc::new(bind_group_layout))
    }

    pub fn get_gpu_bind_group_layout(&self) -> &GpuBindGroupLayout {
        &self.0
    }

    pub fn get_bind_group_layout(&self) -> &RawBindGroupLayout {
        self.0.get_bind_group_layout()
    }
}

#[derive(Clone)]
pub struct PipelineLayout(Arc<GpuPipelineLayout>);

impl PipelineLayout {
    pub fn new(
        device: &RenderDevice,
        descs: &[BindGroupLayoutDescriptor],
        bind_group_layout_cache: &mut FxHashMap<BindGroupLayoutDescriptor, BindGroupLayout>,
    ) -> Self {
        let mut layouts = vec![];

        for desc in descs.iter() {
            bind_group_layout_cache
                .entry(desc.clone())
                .or_insert_with(|| BindGroupLayout::new(device, desc));

            layouts.push(
                bind_group_layout_cache
                    .get(desc)
                    .unwrap()
                    .get_gpu_bind_group_layout()
                    .clone(),
            );
        }

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: layouts.iter().collect(),
        });

        Self(Arc::new(pipeline_layout))
    }

    pub fn get_pipeline_layout(&self) -> &RawPipelineLayout {
        self.0.get_pipeline_layout()
    }
}
