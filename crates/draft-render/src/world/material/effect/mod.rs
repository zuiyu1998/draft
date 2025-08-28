mod resource_bindings;

use std::fmt::Display;

use draft_gfx_base::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind, RenderDevice, RenderQueue,
};
pub use resource_bindings::*;

use crate::{
    BindGroupLayout, FrameworkError, MaterialError, MaterialPropertyGroup, MaterialResourceBinding,
    MaterialResourceHandle, MaterialSamplerHandle, MaterialTextureBinding, MaterialTextureHandle,
    PipelineCache, TextureCache,
};
use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};

#[derive(Default)]
pub struct MaterialEffectInfo {
    pub effect_name: ImmutableString,
    pub resource_binding_definitions: Vec<ResourceBindingDefinition>,
}

impl MaterialEffectInfo {
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
        effect: &MaterialEffect,
        context: &mut MaterialEffectContext,
    ) -> Result<MaterialEffectData, FrameworkError> {
        let desc = self.to_bind_group_layout_descriptor();

        let bind_group_layout = context
            .pipeline_cache
            .get_or_create_bind_group_layout(&desc)
            .clone();

        let mut handles = vec![];

        for resource_binding_definition in self.resource_binding_definitions.iter() {
            handles.push(resource_binding_definition.extra(&effect.resource_bindings, context)?);
        }

        Ok(MaterialEffectData {
            bind_group_layout,
            handles,
        })
    }
}

pub struct ResourceBindingDefinition {
    pub name: ResourceBindingName,
    pub entry: BindGroupLayoutEntry,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialEffect {
    pub effect_name: ImmutableString,
    pub resource_bindings: ResourceBindings,
}

impl MaterialEffect {
    pub fn new(info: &MaterialEffectInfo) -> Self {
        let mut resource_bindings = ResourceBindings::default();

        for resource_binding_definition in info.resource_binding_definitions.iter() {
            match &resource_binding_definition.name {
                ResourceBindingName::Global(name) => {
                    resource_bindings.insert(
                        ResourceBindingName::Global(name.clone()),
                        MaterialResourceBinding::BuiltIn,
                    );
                }
                ResourceBindingName::Local(name) => {
                    let kind = resource_binding_definition.entry.ty.get_binding_type_kind();

                    match kind {
                        BindingTypeKind::Sampler | BindingTypeKind::Texture => {
                            resource_bindings.insert(
                                ResourceBindingName::Local(name.clone()),
                                MaterialResourceBinding::Texture(MaterialTextureBinding::default()),
                            );
                        }
                        BindingTypeKind::Buffer => {
                            resource_bindings.insert(
                                ResourceBindingName::Local(name.clone()),
                                MaterialResourceBinding::PropertyGroup(
                                    MaterialPropertyGroup::default(),
                                ),
                            );
                        }
                        _ => {}
                    }
                }
            }
        }

        Self {
            effect_name: info.effect_name.clone(),
            resource_bindings,
        }
    }
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
        texture: texture_data.get_texture(),
    })
}

fn extra_sampler(
    resource_binding: &MaterialTextureBinding,
    device: &RenderDevice,
    queue: &RenderQueue,
    texture_cache: &mut TextureCache,
) -> Result<MaterialSamplerHandle, FrameworkError> {
    let texture = resource_binding.value.as_ref().unwrap();
    let texture_data = texture_cache.get_or_create(device, queue, texture)?;

    Ok(MaterialSamplerHandle {
        sampler: texture_data.sampler.clone(),
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
            BindingTypeKind::Sampler => {
                let resource_binding = resource_bindings.get(&self.name).ok_or(
                    MaterialError::ResourceBindingDefinitionNotFound {
                        name: self.name.to_string(),
                    },
                )?;

                if let MaterialResourceBinding::Texture(resource_binding) = resource_binding {
                    Ok(MaterialResourceHandle::Sampler(extra_sampler(
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

#[derive(Default)]
pub struct MaterialEffectInfoContainer(FxHashMap<ImmutableString, MaterialEffectInfo>);

impl MaterialEffectInfoContainer {
    pub fn get(&self, name: &ImmutableString) -> Option<&MaterialEffectInfo> {
        self.0.get(name)
    }

    pub fn register_material_effect_info(&mut self, effect_info: MaterialEffectInfo) {
        self.0.insert(effect_info.effect_name.clone(), effect_info);
    }
}

pub struct MaterialEffectData {
    pub bind_group_layout: BindGroupLayout,
    pub handles: Vec<MaterialResourceHandle>,
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
