mod resource_bindings;

use std::ops::{Deref, DerefMut};

use draft_gfx_base::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind};
pub use resource_bindings::*;

use crate::{
    BindGroupLayout, FrameworkError, MaterialResourceHandle, MaterialTextureHandle, PipelineCache,
};
use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialEffect {
    pub effect_name: ImmutableString,
    pub resource_bindings: ResourceBindings,
}

pub struct ResourceBindingDefinition {
    pub name: ResourceBindingName,
    pub entry: BindGroupLayoutEntry,
}

fn extra_texture() -> MaterialTextureHandle {
    todo!()
}

impl ResourceBindingDefinition {
    pub fn extra(
        &self,
        resource_bindings: &ResourceBindings,
        _context: &mut MaterialEffectContext,
    ) -> Result<MaterialResourceHandle, FrameworkError> {
        let kind = self.entry.ty.get_binding_type_kind();

        match kind {
            BindingTypeKind::Texture => {
                let _resource_binding = resource_bindings.get(&self.name);

                Ok(MaterialResourceHandle::Texture(extra_texture()))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub struct MaterialEffectProcessorContainer(FxHashMap<ImmutableString, MaterialEffectProcessor>);

impl Deref for MaterialEffectProcessorContainer {
    type Target = FxHashMap<ImmutableString, MaterialEffectProcessor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialEffectProcessorContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MaterialEffectProcessor {
    pub name: ImmutableString,
    pub resource_binding_definitions: Vec<ResourceBindingDefinition>,
}

impl MaterialEffectProcessor {
    pub fn to_bind_group_layout_descriptor(&self) -> BindGroupLayoutDescriptor {
        BindGroupLayoutDescriptor {
            entries: self
                .resource_binding_definitions
                .iter()
                .map(|definition| definition.entry.clone())
                .collect(),
        }
    }

    pub fn process(
        &self,
        _effct: &MaterialEffect,
        context: &mut MaterialEffectContext,
    ) -> MaterialEffectData {
        let desc = self.to_bind_group_layout_descriptor();

        let bind_group_layout = context
            .pipeline_cache
            .get_or_create_bind_group_layout(&desc)
            .clone();

        MaterialEffectData { bind_group_layout }
    }
}

pub struct MaterialEffectData {
    pub bind_group_layout: BindGroupLayout,
}

pub struct MaterialEffectContext<'a> {
    pub pipeline_cache: &'a mut PipelineCache,
}

#[derive(Debug, Clone, Reflect, Visit, PartialEq, Eq, Hash)]
pub enum ResourceBindingName {
    Global(ImmutableString),
    Local(ImmutableString),
}

impl Default for ResourceBindingName {
    fn default() -> Self {
        ResourceBindingName::Local("".into())
    }
}
