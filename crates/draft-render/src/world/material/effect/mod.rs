mod container;
mod loader;
mod resource_bindings;

pub use container::*;
pub use loader::*;
pub use resource_bindings::*;

use std::{error::Error, fs::File, io::Write, path::Path};

use crate::{
    BindGroupLayout, FrameworkError, MaterialError, MaterialResourceBinding,
    MaterialResourceHandle, MaterialSamplerHandle, MaterialTextureBinding, MaterialTextureHandle,
    PipelineCache, TextureCache,
    gfx_base::{
        BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind, RenderDevice, RenderQueue,
    },
};
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData, io::ResourceIo};
use serde::{Deserialize, Serialize};

pub type MaterialEffectResource = Resource<MaterialEffect>;

pub struct MaterialBindGroupHandle {
    pub bind_group_layout: BindGroupLayout,
    pub material_resource_handles: Vec<MaterialResourceHandle>,
}

#[derive(Default, Reflect, Visit, Clone, Debug, TypeUuidProvider, Deserialize, Serialize)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct MaterialEffect {
    pub effect_name: ImmutableString,
    pub resource_binding_definitions: Vec<ResourceBindingDefinition>,
}

#[derive(Default, Reflect, Visit, Debug, Clone, Deserialize, Serialize)]
pub struct MaterialEffectInfo {
    pub effect_name: ImmutableString,
}

impl ResourceData for MaterialEffect {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let toml = toml::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(toml.as_bytes())?;

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
    pub async fn from_file<P>(path: P, io: &dyn ResourceIo) -> Result<Self, FrameworkError>
    where
        P: AsRef<Path>,
    {
        let content = io.load_file(path.as_ref()).await?;
        let effect = toml::from_slice::<Self>(&content)?;

        Ok(effect)
    }

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

#[derive(Default, Reflect, Visit, Debug, Clone, Deserialize, Serialize)]
pub struct ResourceBindingDefinition {
    pub name: ImmutableString,
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
        resource_bindings: &ResourceBindings,
    ) -> Result<MaterialBindGroupHandle, FrameworkError> {
        let desc = effect.to_bind_group_layout_descriptor();

        let bind_group_layout = self
            .pipeline_cache
            .get_or_create_bind_group_layout(&desc)
            .clone();

        let mut handles = vec![];

        for resource_binding_definition in effect.resource_binding_definitions.iter() {
            handles.push(resource_binding_definition.extra(resource_bindings, self)?);
        }

        Ok(MaterialBindGroupHandle {
            bind_group_layout,
            material_resource_handles: handles,
        })
    }
}
