use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    FrameworkError, NamedValue, NamedValuesContainer,
    gfx_base::{
        BindGroupLayoutEntry, RawBindGroupLayout, RawPipelineLayout, RawPipelineLayoutDescriptor,
        RenderDevice,
    },
};

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq)]
pub struct PipelineLayoutDescriptor(FxHashMap<ImmutableString, BindGroupLayoutDescriptor>);

impl Deref for PipelineLayoutDescriptor {
    type Target = FxHashMap<ImmutableString, BindGroupLayoutDescriptor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hash for PipelineLayoutDescriptor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (name, desc) in self.0.iter() {
            name.hash(state);
            desc.hash(state);
        }
    }
}

impl PipelineLayoutDescriptor {
    pub fn get_bind_group_layouts(&self) -> NamedValuesContainer<BindGroupLayoutDescriptor> {
        let bind_group_layouts = self
            .0
            .iter()
            .map(|(name, value)| NamedValue {
                name: name.clone(),
                value: value.clone(),
            })
            .collect::<Vec<NamedValue<BindGroupLayoutDescriptor>>>();

        NamedValuesContainer::from(bind_group_layouts)
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
    ) -> Result<&BindGroupLayout, FrameworkError> {
        if !self.bind_group_layout_cache.contains_key(desc) {
            let bind_group_layout = BindGroupLayout::new(device, desc)?;
            self.bind_group_layout_cache
                .insert(desc.clone(), bind_group_layout);
        }

        Ok(self.bind_group_layout_cache.get(desc).unwrap())
    }

    pub fn get_or_create_pipeline_layout(
        &mut self,
        device: &RenderDevice,
        desc: &PipelineLayoutDescriptor,
    ) -> Result<&PipelineLayout, FrameworkError> {
        if !self.pipeline_layout_cache.contains_key(desc) {
            let mut bind_group_layouts = FxHashMap::default();
            let named_values_container = desc.get_bind_group_layouts();

            for bind_group_layout_desc in named_values_container.iter() {
                let data =
                    self.get_or_create_bind_group_layout(device, &bind_group_layout_desc.value)?;

                bind_group_layouts.insert(bind_group_layout_desc.name.clone(), data.clone());
            }

            let raw_bind_group_layouts = bind_group_layouts
                .values()
                .map(|value| value.raw())
                .collect::<Vec<&RawBindGroupLayout>>();

            let layout =
                device
                    .wgpu_device()
                    .create_pipeline_layout(&RawPipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &raw_bind_group_layouts,
                        push_constant_ranges: &[],
                    });

            let layout = PipelineLayout::new(layout);
            self.pipeline_layout_cache.insert(desc.clone(), layout);
        }

        Ok(self.pipeline_layout_cache.get(desc).unwrap())
    }

    pub fn get(&mut self, desc: &PipelineLayoutDescriptor) -> Option<&PipelineLayout> {
        self.pipeline_layout_cache.get(desc)
    }
}

#[derive(Clone)]
pub struct BindGroupLayout(Arc<RawBindGroupLayout>);

impl BindGroupLayout {
    pub fn raw(&self) -> &RawBindGroupLayout {
        &self.0
    }

    pub fn new(
        device: &RenderDevice,
        desc: &BindGroupLayoutDescriptor,
    ) -> Result<Self, FrameworkError> {
        let entries = desc
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

        Ok(BindGroupLayout(Arc::new(bind_group_layout)))
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
