use fyrox_core::{reflect::*, sparse::AtomicIndex, visitor::*};
use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    FrameworkError, TemporaryCache,
    gfx_base::{BindGroupLayoutEntry, RawBindGroupLayout, RenderDevice},
};

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayoutDescriptor>,
}

#[derive(Default)]
pub struct PipelineLayoutCache {
    pub pipeline_layout_map: HashMap<PipelineLayoutDescriptor, PipelineLayout>,
    pub pipeline_layout_cache: TemporaryCache<PipelineLayoutData>,
    pub bind_group_layout_cache: TemporaryCache<BindGroupLayoutData>,
    pub bind_group_layout_map: HashMap<BindGroupLayoutDescriptor, BindGroupLayout>,
}

impl PipelineLayoutCache {
    pub fn get_or_insert_bind_group_layout_data(
        &mut self,
        device: &RenderDevice,
        desc: &BindGroupLayoutDescriptor,
    ) -> Result<&BindGroupLayoutData, FrameworkError> {
        let layout = self.get_or_insert_bind_group_layout(desc).clone();

        self.bind_group_layout_cache.get_or_insert_with(
            &layout.cache_index,
            Default::default(),
            || BindGroupLayoutData::new(device, &layout),
        )
    }

    pub fn get_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor,
    ) -> Option<&RawBindGroupLayout> {
        self.bind_group_layout_map.get(desc).and_then(|layout| {
            self.bind_group_layout_cache
                .buffer
                .get(&layout.cache_index)
                .map(|entry| entry.bind_group_layout.deref())
        })
    }

    pub fn get_or_insert_bind_group_layout(
        &mut self,
        desc: &BindGroupLayoutDescriptor,
    ) -> &BindGroupLayout {
        if !self.bind_group_layout_map.contains_key(desc) {
            let layout = BindGroupLayout::new(desc.clone());
            self.bind_group_layout_map.insert(desc.clone(), layout);
        }

        self.bind_group_layout_map.get(desc).unwrap()
    }

    pub fn get_pipeline_layout(
        &mut self,
        device: &RenderDevice,
        desc: &PipelineLayoutDescriptor,
    ) -> Result<&PipelineLayout, FrameworkError> {
        if !self.pipeline_layout_map.contains_key(desc) {
            let mut bind_group_layouts = vec![];

            for bind_group_layout in desc.bind_group_layouts.iter() {
                let data = self.get_or_insert_bind_group_layout_data(device, bind_group_layout)?;

                bind_group_layouts.push(data.clone());
            }

            let layout = PipelineLayout::new(desc.clone(), bind_group_layouts);
            self.pipeline_layout_map.insert(desc.clone(), layout);
        }

        Ok(self.pipeline_layout_map.get(desc).unwrap())
    }

    pub fn get(
        &mut self,
        device: &RenderDevice,
        desc: &PipelineLayoutDescriptor,
    ) -> Result<&wgpu::PipelineLayout, FrameworkError> {
        let layout = self.get_pipeline_layout(device, desc)?.clone();

        match self.pipeline_layout_cache.get_or_insert_with(
            &layout.cache_index,
            Default::default(),
            || PipelineLayoutData::new(device, &layout),
        ) {
            Ok(data) => Ok(&data.layout),
            Err(error) => Err(error),
        }
    }
}

#[derive(Clone)]
pub struct BindGroupLayout {
    pub desc: BindGroupLayoutDescriptor,
    pub cache_index: Arc<AtomicIndex>,
}

impl BindGroupLayout {
    pub fn new(desc: BindGroupLayoutDescriptor) -> Self {
        Self {
            desc,
            cache_index: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct BindGroupLayoutData {
    pub bind_group_layout: Arc<wgpu::BindGroupLayout>,
}

impl BindGroupLayoutData {
    pub fn new(device: &RenderDevice, layout: &BindGroupLayout) -> Result<Self, FrameworkError> {
        let entries = layout
            .desc
            .entries
            .clone()
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        let bind_group_layout =
            device
                .wgpu_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });

        Ok(BindGroupLayoutData {
            bind_group_layout: Arc::new(bind_group_layout),
        })
    }
}

pub struct PipelineLayoutData {
    pub layout: Arc<wgpu::PipelineLayout>,
}

impl PipelineLayoutData {
    pub fn new(device: &RenderDevice, layout: &PipelineLayout) -> Result<Self, FrameworkError> {
        let bind_group_layouts = layout
            .bind_group_layouts
            .iter()
            .map(|data| data.bind_group_layout.as_ref())
            .collect::<Vec<&wgpu::BindGroupLayout>>();

        let layout = device
            .wgpu_device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bind_group_layouts,
                //todo PushConstantRange
                push_constant_ranges: &[],
            });
        Ok(PipelineLayoutData {
            layout: Arc::new(layout),
        })
    }
}

#[derive(Clone)]
pub struct PipelineLayout {
    pub desc: PipelineLayoutDescriptor,
    pub bind_group_layouts: Vec<BindGroupLayoutData>,
    pub cache_index: Arc<AtomicIndex>,
}

impl PipelineLayout {
    pub fn new(
        desc: PipelineLayoutDescriptor,
        bind_group_layouts: Vec<BindGroupLayoutData>,
    ) -> Self {
        Self {
            desc,
            bind_group_layouts,
            cache_index: Default::default(),
        }
    }
}
