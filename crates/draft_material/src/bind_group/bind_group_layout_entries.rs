use std::num::NonZero;

use draft_graphics::{BindGroupLayoutEntry, BindingType, ShaderStages};
use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

pub trait IntoBindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder;
}

impl IntoBindGroupLayoutEntryBuilder for BindingType {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        BindGroupLayoutEntryBuilder {
            ty: self,
            visibility: None,
            count: 0,
        }
    }
}

impl IntoBindGroupLayoutEntryBuilder for BindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        self
    }
}

#[derive(Clone)]
pub struct BindGroupLayoutEntryBuilder {
    ty: BindingType,
    visibility: Option<ShaderStages>,
    count: u32,
}

impl BindGroupLayoutEntryBuilder {
    pub fn visibility(mut self, visibility: ShaderStages) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn build(
        &self,
        name: &str,
        binding: u32,
        default_visibility: ShaderStages,
    ) -> MaterialBindGroupLayoutEntry {
        MaterialBindGroupLayoutEntry {
            binding,
            ty: self.ty.clone(),
            visibility: self.visibility.unwrap_or(default_visibility),
            count: self.count,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStages,
    pub ty: BindingType,
    pub count: u32,
    pub name: String,
}

impl MaterialBindGroupLayoutEntry {
    pub fn get_bind_group_layout_entry(&self) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility.get_wgpu_shader_stages(),
            ty: self.ty.get_binding_type(),
            count: NonZero::new(self.count),
        }
    }
}

pub mod binding_types {
    use crate::{BindGroupLayoutEntryBuilder, IntoBindGroupLayoutEntryBuilder};
    use draft_graphics::{BindingType, BufferBindingType};
    use encase::ShaderType;

    pub fn storage_buffer_read_only<T: ShaderType>(
        has_dynamic_offset: bool,
    ) -> BindGroupLayoutEntryBuilder {
        storage_buffer_read_only_sized(has_dynamic_offset, T::min_size().get())
    }

    pub fn storage_buffer_read_only_sized(
        has_dynamic_offset: bool,
        min_binding_size: u64,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset,
            min_binding_size,
        }
        .into_bind_group_layout_entry_builder()
    }
}
