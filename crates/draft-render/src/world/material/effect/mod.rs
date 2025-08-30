mod container;
mod resource_bindings;

pub use container::*;
pub use resource_bindings::*;

use std::{error::Error, path::Path};

use draft_gfx_base::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind, RenderDevice, RenderQueue,
};

use crate::{
    BindGroupLayout, FrameworkError, MaterialError, MaterialPropertyGroup, MaterialResourceBinding,
    MaterialResourceHandle, MaterialSamplerHandle, MaterialTextureBinding, MaterialTextureHandle,
    PipelineCache, TextureCache,
};
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};

pub type MaterialEffectResource = Resource<MaterialEffect>;

#[derive(Default, Reflect, Visit, Clone, Debug, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct MaterialEffect {
    pub effect_name: ImmutableString,
    pub resource_binding_definitions: Vec<ResourceBindingDefinition>,
}

pub struct MaterialEffectInfo {
    pub effect_name: ImmutableString,
}

impl ResourceData for MaterialEffect {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("MaterialEffect", &mut visitor)?;
        visitor.save_binary_to_file(path)?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

impl MaterialEffect {
    pub fn to_bind_group_layout_descriptor(&self) -> BindGroupLayoutDescriptor {
        BindGroupLayoutDescriptor {
            entries: self
                .resource_binding_definitions
                .iter()
                .map(|definition| definition.entry.clone())
                .collect(),
        }
    }
}

#[derive(Default, Reflect, Visit, Debug, Clone)]
pub struct ResourceBindingDefinition {
    pub name: ImmutableString,
    pub entry: BindGroupLayoutEntry,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialEffectInstance {
    pub effect_name: ImmutableString,
    pub resource_bindings: ResourceBindings,
}

impl MaterialEffectInstance {
    pub fn new(info: &MaterialEffectInfo, container: &MaterialEffectContainer) -> Self {
        let effect = container
            .get(&info.effect_name)
            .expect("material_effect mut have");

        let mut resource_bindings = ResourceBindings::default();

        for resource_binding_definition in effect.resource_binding_definitions.iter() {
            let kind = resource_binding_definition.entry.ty.get_binding_type_kind();

            match kind {
                BindingTypeKind::Sampler | BindingTypeKind::Texture => {
                    resource_bindings.insert(
                        resource_binding_definition.name.clone(),
                        MaterialResourceBinding::Texture(MaterialTextureBinding::default()),
                    );
                }
                BindingTypeKind::Buffer => {
                    resource_bindings.insert(
                        resource_binding_definition.name.clone(),
                        MaterialResourceBinding::PropertyGroup(MaterialPropertyGroup::default()),
                    );
                }
                _ => {}
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

impl MaterialEffectContext<'_> {
    pub fn process(
        &mut self,
        effect: &MaterialEffect,
        instance: &MaterialEffectInstance,
    ) -> Result<MaterialEffectData, FrameworkError> {
        let desc = effect.to_bind_group_layout_descriptor();

        let bind_group_layout = self
            .pipeline_cache
            .get_or_create_bind_group_layout(&desc)
            .clone();

        let mut handles = vec![];

        for resource_binding_definition in effect.resource_binding_definitions.iter() {
            handles.push(resource_binding_definition.extra(&instance.resource_bindings, self)?);
        }

        Ok(MaterialEffectData {
            bind_group_layout,
            handles,
        })
    }
}
