use draft_graphics::ShaderStages;
use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

use crate::{IntoBindGroupLayoutEntryBuilder, MaterialBindGroupLayoutEntry};

pub struct MaterialBindGroupLayoutBuilder {
    name: String,
    entries: Vec<Option<MaterialBindGroupLayoutEntry>>,
    default_visibility: ShaderStages,
}

impl MaterialBindGroupLayoutBuilder {
    pub fn new(name: &str, default_visibility: ShaderStages, size: usize) -> Self {
        Self {
            name: name.to_string(),
            entries: vec![None; size],
            default_visibility,
        }
    }

    pub fn with<T: IntoBindGroupLayoutEntryBuilder>(
        mut self,
        name: &str,
        binding: usize,
        value: T,
    ) -> Self {
        let builder = value.into_bind_group_layout_entry_builder();
        let entry = builder.build(name, binding as u32, self.default_visibility);

        self.entries[binding] = Some(entry);

        self
    }

    pub fn build(self) -> MaterialBindGroupLayout {
        let mut entries = vec![];

        self.entries
            .into_iter()
            .enumerate()
            .for_each(|(index, entry)| {
                if entry.is_none() {
                    panic!("Material bind group layout {} must have entry.", index);
                } else {
                    entries.push(entry.unwrap());
                }
            });

        MaterialBindGroupLayout {
            name: self.name,
            entries,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroupLayout {
    pub name: String,
    pub entries: Vec<MaterialBindGroupLayoutEntry>,
}
