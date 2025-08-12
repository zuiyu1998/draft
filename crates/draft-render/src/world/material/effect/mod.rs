mod resource_bindings;

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use draft_gfx_base::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind, RenderDevice, RenderQueue,
};
pub use resource_bindings::*;

use crate::{
    BindGroupLayout, FrameworkError, MaterialError, MaterialResourceBinding,
    MaterialResourceHandle, MaterialTextureBinding, MaterialTextureHandle, PipelineCache,
    TextureCache,
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

fn extra_texture(
    resource_binding: &MaterialTextureBinding,
    device: &RenderDevice,
    queue: &RenderQueue,
    texture_cache: &mut TextureCache,
) -> Result<MaterialTextureHandle, FrameworkError> {
    let texture = resource_binding.value.as_ref().unwrap();
    let texture_data = texture_cache.get_or_create(device, queue, texture)?;

    Ok(MaterialTextureHandle {
        texture: texture_data.render_data.texture.clone(),
    })
}

impl ResourceBindingDefinition {
    pub fn extra(
        &self,
        resource_bindings: &ResourceBindings,
        context: &mut MaterialEffectContext,
    ) -> Result<MaterialResourceHandle, FrameworkError> {
        let kind = self.entry.ty.get_binding_type_kind();

        match kind {
            BindingTypeKind::Texture => {
                let resource_binding = resource_bindings.get(&self.name).ok_or(
                    MaterialError::ResourceBindingDefinitionNotFound {
                        name: self.name.to_string(),
                    },
                )?;

                if let MaterialResourceBinding::Texture(resource_binding) = resource_binding {
                    Ok(MaterialResourceHandle::Texture(extra_texture(
                        resource_binding,
                        context.device,
                        context.queue,
                        context.texture_cache,
                    )?))
                } else {
                    let target_kind = resource_binding.get_binding_type_kind();

                    Err(MaterialError::ResourceBindingDefinitionNotMatch {
                        name: self.name.to_string(),
                        target_kind,
                        source_kind: kind,
                    }
                    .into())
                }
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
    pub device: &'a RenderDevice,
    pub queue: &'a RenderQueue,
    pub texture_cache: &'a mut TextureCache,
}

#[derive(Debug, Clone, Reflect, Visit, PartialEq, Eq, Hash)]
pub enum ResourceBindingName {
    Global(ImmutableString),
    Local(ImmutableString),
}

impl Display for ResourceBindingName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceBindingName::Global(name) => f.write_fmt(format_args!("global:{name}")),
            ResourceBindingName::Local(name) => f.write_fmt(format_args!("local:{name}")),
        }
    }
}

impl Default for ResourceBindingName {
    fn default() -> Self {
        ResourceBindingName::Local("".into())
    }
}
