mod entry;

pub use entry::*;

use crate::WgpuBindGroupLayout;
use fyrox_core::{reflect::*, visitor::*};
use std::collections::HashMap;

pub struct BindGroupLayoutDescriptorBuilder {
    entries: Vec<BindGroupLayoutEntry>,
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
        binding: u32,
        bind_group_layout: BindGroupLayoutEntryBuilder,
    ) {
        let bind_group_layout_entry = bind_group_layout.build(binding, self.default_visibility);

        if let Some(index) = self.binding_to_entries.get(&binding) {
            self.entries[*index] = bind_group_layout_entry;
        } else {
            let index = self.entries.len();
            self.entries.push(bind_group_layout_entry);
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
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Clone)]
pub struct GpuBindGroupLayout(WgpuBindGroupLayout);

impl GpuBindGroupLayout {
    pub fn new(bind_group_layout: WgpuBindGroupLayout) -> Self {
        GpuBindGroupLayout(bind_group_layout)
    }

    pub fn get_bind_group_layout(&self) -> &WgpuBindGroupLayout {
        &self.0
    }
}
