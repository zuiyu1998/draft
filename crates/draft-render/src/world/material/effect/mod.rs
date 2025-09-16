mod container;
mod loader;
mod resource_bindings;

pub use container::*;
pub use loader::*;
pub use resource_bindings::*;

use std::{error::Error, fs::File, io::Write, num::NonZero, path::Path};

use crate::{
    BindGroupLayout, FrameContext, FrameworkError, MaterialBufferHandle, MaterialError,
    MaterialResourceBinding, MaterialResourceHandle, MaterialSamplerHandle, MaterialTextureBinding,
    MaterialTextureHandle, RenderWorld, TextureCache,
};
use draft_gfx_base::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingTypeKind, RenderDevice, RenderQueue,
    TextureViewDescriptor,
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

impl MaterialEffect {
    pub fn process(
        &self,
        mut context: MaterialEffectContext,
    ) -> Result<MaterialBindGroupHandle, FrameworkError> {
        let mut material_resource_handles = vec![];
        let mut entries = vec![];

        for definition in self.resource_binding_definitions.iter() {
            material_resource_handles.push(definition.extra(&mut context)?);
            entries.push(definition.entry.clone());
        }

        let desc = BindGroupLayoutDescriptor { entries };

        let layout = context
            .world
            .pipeline_cache
            .get_or_create_bind_group_layout(&desc)
            .clone();

        Ok(MaterialBindGroupHandle {
            bind_group_layout: layout,
            material_resource_handles,
        })
    }
}

pub struct MaterialEffectContext<'a> {
    pub resource_bindings: &'a ResourceBindings,
    pub frame_context: &'a FrameContext,
    pub camera_offset: u32,
    pub camera_size: NonZero<u64>,
    pub world: &'a mut RenderWorld,
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
        let toml = serde_yml::to_string(self)?;
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
        let effect = serde_yml::from_slice::<Self>(&content)?;

        Ok(effect)
    }

    pub fn get_bind_group_layout_descriptor(&self) -> BindGroupLayoutDescriptor {
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
        texture_view_desc: TextureViewDescriptor::default(),
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
    pub fn is_internal(&self) -> bool {
        self.name.starts_with("internal/")
    }

    pub fn extra(
        &self,
        context: &mut MaterialEffectContext,
    ) -> Result<MaterialResourceHandle, FrameworkError> {
        if self.is_internal() {
            let name = self.name.as_str();

            match name {
                "internal/camera" => Ok(MaterialResourceHandle::Buffer(MaterialBufferHandle {
                    offset: context.camera_offset,
                    size: Some(context.camera_size),
                    buffer: context.frame_context.camera_uniforms.get_camera_buffer(),
                })),
                _ => {
                    todo!()
                }
            }
        } else {
            let kind = self.entry.ty.get_binding_type_kind();
            let resource_binding = context.resource_bindings.get(&self.name).ok_or(
                MaterialError::ResourceBindingDefinitionNotFound {
                    name: self.name.to_string(),
                },
            )?;

            match kind {
                BindingTypeKind::Texture => {
                    if let MaterialResourceBinding::Texture(resource_binding) = resource_binding {
                        Ok(MaterialResourceHandle::Texture(extra_texture(
                            resource_binding,
                            &context.world.server.device,
                            &context.world.server.queue,
                            &mut context.world.texture_cache,
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
                    if let MaterialResourceBinding::Texture(resource_binding) = resource_binding {
                        Ok(MaterialResourceHandle::Sampler(extra_sampler(
                            resource_binding,
                            &context.world.server.device,
                            &context.world.server.queue,
                            &mut context.world.texture_cache,
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
                    todo!()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use draft_gfx_base::{
        SamplerBindingType, ShaderStages, TextureSampleType,
        binding_types::{sampler, texture_2d},
    };

    use super::ResourceBindingDefinition;
    use crate::MaterialEffect;

    #[test]
    fn test_print_material_effect() {
        let mut material_effect = MaterialEffect {
            effect_name: "test".into(),
            ..MaterialEffect::default()
        };

        material_effect
            .resource_binding_definitions
            .push(ResourceBindingDefinition {
                name: "t_diffuse".into(),
                entry: texture_2d(TextureSampleType::Float { filterable: true })
                    .build(0, ShaderStages::all()),
            });

        material_effect
            .resource_binding_definitions
            .push(ResourceBindingDefinition {
                name: "t_diffuse".into(),
                entry: sampler(SamplerBindingType::Filtering).build(1, ShaderStages::all()),
            });
    }
}
