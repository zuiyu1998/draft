use std::num::NonZero;

use draft_graphics::{BindGroupLayoutEntry, BindingType, ShaderStages};
use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroupLayout {
    pub name: String,
    pub entries: Vec<MaterialBindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroup {
    pub name: String,
    pub layouts: Vec<MaterialBindGroupLayout>,
}
