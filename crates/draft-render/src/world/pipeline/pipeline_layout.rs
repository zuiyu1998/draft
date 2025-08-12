use draft_gfx_base::GpuBindGroupLayout;
use fxhash::FxHashMap;
use fyrox_core::{reflect::*, visitor::*};
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    FrameworkError,
    gfx_base::{BindGroupLayoutDescriptor, RawBindGroupLayout, RawPipelineLayout, RenderDevice},
};

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor(Vec<BindGroupLayoutDescriptor>);

impl PipelineLayoutDescriptor {
    pub fn get_bind_group_layout_descs(&self) -> &[BindGroupLayoutDescriptor] {
        &self.0
    }
}

impl Deref for PipelineLayoutDescriptor {
    type Target = Vec<BindGroupLayoutDescriptor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct PipelineLayoutCache {
    pipeline_layout_cache: FxHashMap<PipelineLayoutDescriptor, PipelineLayout>,
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
        _device: &RenderDevice,
        _desc: &PipelineLayoutDescriptor,
    ) -> Result<&PipelineLayout, FrameworkError> {
        todo!()
    }

    pub fn get(&mut self, desc: &PipelineLayoutDescriptor) -> Option<&PipelineLayout> {
        self.pipeline_layout_cache.get(desc)
    }
}

#[derive(Clone)]
pub struct BindGroupLayout(Arc<GpuBindGroupLayout>);

impl BindGroupLayout {
    pub fn new(device: &RenderDevice, desc: &BindGroupLayoutDescriptor) -> Self {
        let bind_group_layout = device.create_bind_group_layout(desc);

        BindGroupLayout(Arc::new(bind_group_layout))
    }

    pub fn get_bind_group_layout(&self) -> &RawBindGroupLayout {
        self.0.get_bind_group_layout()
    }
}

#[derive(Clone)]
pub struct PipelineLayout(Arc<RawPipelineLayout>);

impl PipelineLayout {
    pub fn new(layout: RawPipelineLayout) -> Self {
        Self(Arc::new(layout))
    }
}

impl Deref for PipelineLayout {
    type Target = RawPipelineLayout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
