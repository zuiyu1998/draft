use std::{borrow::Cow, num::NonZero};

use draft_graphics::{BindGroupLayoutEntry, BindingType, ShaderStages};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BindGroupLayoutDescriptor {
    pub label: Cow<'static, str>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Clone, Copy)]
pub struct BindGroupLayoutEntryBuilder {
    ty: BindingType,
    visibility: Option<ShaderStages>,
    count: Option<NonZero<u32>>,
}

pub trait IntoBindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder;
}

impl BindGroupLayoutEntryBuilder {
    pub fn visibility(mut self, visibility: ShaderStages) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn count(mut self, count: NonZero<u32>) -> Self {
        self.count = Some(count);
        self
    }

    pub fn build(&self, binding: u32, default_visibility: ShaderStages) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            ty: self.ty,
            visibility: self.visibility.unwrap_or(default_visibility),
            count: self.count,
        }
    }
}

pub struct BindGroupLayoutEntries {
    default_visibility: ShaderStages,
    entries: Vec<BindGroupLayoutEntryBuilder>,
}

impl BindGroupLayoutEntries {
    pub fn new(default_visibility: ShaderStages) -> Self {
        Self {
            default_visibility,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: BindGroupLayoutEntryBuilder) -> &mut Self {
        self.entries.push(entry);
        self
    }

    pub fn build(&self) -> Vec<BindGroupLayoutEntry> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, entry)| entry.build(i as u32, self.default_visibility))
            .collect()
    }
}

impl IntoBindGroupLayoutEntryBuilder for BindingType {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder {
        BindGroupLayoutEntryBuilder {
            ty: self,
            visibility: None,
            count: None,
        }
    }
}

pub mod binding_types {
    use std::num::NonZero;

    use crate::render_resource::{BindGroupLayoutEntryBuilder, IntoBindGroupLayoutEntryBuilder};
    use draft_graphics::{BindingType, BufferBindingType};
    use encase::ShaderType;

    pub fn uniform_buffer<T: ShaderType>(has_dynamic_offset: bool) -> BindGroupLayoutEntryBuilder {
        uniform_buffer_sized(has_dynamic_offset, Some(T::min_size()))
    }

    pub fn uniform_buffer_sized(
        has_dynamic_offset: bool,
        min_binding_size: Option<NonZero<u64>>,
    ) -> BindGroupLayoutEntryBuilder {
        BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset,
            min_binding_size,
        }
        .into_bind_group_layout_entry_builder()
    }
}
