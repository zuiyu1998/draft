use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    FrameworkError,
    gfx_base::{
        BindGroupLayoutEntry, BindGroupLayoutEntryBuilder, RawBindGroupLayout, RawPipelineLayout,
        RawPipelineLayoutDescriptor, RenderDevice, ShaderStages,
    },
};

pub struct BindGroupLayoutDescriptorBuilder {
    entries: Vec<BindGroupLayoutEntryDescriptor>,
    default_visibility: ShaderStages,
    binding_to_entries: HashMap<u32, usize>,
}

impl BindGroupLayoutDescriptorBuilder {
    pub fn new(default_visibility: ShaderStages) -> BindGroupLayoutDescriptorBuilder {
        BindGroupLayoutDescriptorBuilder {
            entries: vec![],
            default_visibility,
            binding_to_entries: HashMap::default(),
        }
    }

    pub fn add_bind_group_layout(
        &mut self,
        key: ImmutableString,
        binding: u32,
        bind_group_layout: BindGroupLayoutEntryBuilder,
    ) {
        let bind_group_layout_entry = bind_group_layout.build(binding, self.default_visibility);

        let bind_group_layout = BindGroupLayoutEntryDescriptor {
            entry: bind_group_layout_entry,
            key,
        };

        if let Some(index) = self.binding_to_entries.get(&binding) {
            self.entries[*index] = bind_group_layout;
        } else {
            let index = self.entries.len();
            self.entries.push(bind_group_layout);
            self.binding_to_entries.insert(binding, index);
        }
    }

    pub fn build(self) -> BindGroupLayoutDescriptor {
        BindGroupLayoutDescriptor {
            entries: self.entries,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub entries: Vec<BindGroupLayoutEntryDescriptor>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutEntryDescriptor {
    pub entry: BindGroupLayoutEntry,
    pub key: ImmutableString,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor(Vec<BindGroupLayoutDescriptor>);

pub struct ResourceKeyContainer {
    pub keys: Vec<ImmutableString>,
}

impl PipelineLayoutDescriptor {
    pub fn get_bind_group_layout_descs(&self) -> &[BindGroupLayoutDescriptor] {
        &self.0
    }

    pub fn get_bind_group_layout_names(&self) -> Vec<ResourceKeyContainer> {
        self.0
            .iter()
            .map(|desc| {
                let mut keys = vec![];
                for entry in desc.entries.iter() {
                    if !keys.contains(&entry.key) {
                        keys.push(entry.key.clone());
                    }
                }

                ResourceKeyContainer { keys }
            })
            .collect()
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
            let mut bind_group_layouts = vec![];

            for bind_group_layout_desc in desc.0.iter() {
                let data = self
                    .get_or_create_bind_group_layout(device, bind_group_layout_desc)?
                    .clone();
                bind_group_layouts.push(data);
            }

            let mut raw_bind_group_layouts = vec![];

            for bind_group_layout in bind_group_layouts.iter() {
                raw_bind_group_layouts.push(bind_group_layout.raw());
            }

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
            .map(|v| v.entry.into())
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
