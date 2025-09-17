use std::num::NonZero;

use draft_render::{
    FrameworkError, MaterialBindGroupHandle, MaterialBufferHandle, MaterialEffect, MaterialError,
    MaterialResourceBinding, MaterialResourceHandle, MaterialSamplerHandle, MaterialTextureBinding,
    MaterialTextureHandle, RenderWorld, ResourceBindingDefinition, ResourceBindings, TextureCache,
    gfx_base::{
        BindGroupLayoutDescriptor, BindingTypeKind, RenderDevice, RenderQueue,
        TextureViewDescriptor,
    },
};

use crate::renderer::FrameContext;

pub struct MaterialEffectContext<'a> {
    pub resource_bindings: &'a ResourceBindings,
    pub frame_context: &'a FrameContext,
    pub camera_offset: u32,
    pub camera_size: NonZero<u64>,
    pub world: &'a mut RenderWorld,
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

impl MaterialEffectContext<'_> {
    pub fn extra(
        &mut self,
        resource_binding_definition: &ResourceBindingDefinition,
    ) -> Result<MaterialResourceHandle, FrameworkError> {
        if resource_binding_definition.is_internal() {
            let name = resource_binding_definition.name.as_str();

            match name {
                "internal/camera" => Ok(MaterialResourceHandle::Buffer(MaterialBufferHandle {
                    offset: self.camera_offset,
                    size: Some(self.camera_size),
                    buffer: self.frame_context.camera_uniforms.get_camera_buffer(),
                })),
                _ => {
                    todo!()
                }
            }
        } else {
            let kind = resource_binding_definition.entry.ty.get_binding_type_kind();
            let resource_binding = self
                .resource_bindings
                .get(&resource_binding_definition.name)
                .ok_or(MaterialError::ResourceBindingDefinitionNotFound {
                    name: resource_binding_definition.name.to_string(),
                })?;

            match kind {
                BindingTypeKind::Texture => {
                    if let MaterialResourceBinding::Texture(resource_binding) = resource_binding {
                        Ok(MaterialResourceHandle::Texture(extra_texture(
                            resource_binding,
                            &self.world.server.device,
                            &self.world.server.queue,
                            &mut self.world.texture_cache,
                        )?))
                    } else {
                        let target_kind = resource_binding.get_binding_type_kind();

                        Err(MaterialError::ResourceBindingDefinitionNotMatch {
                            name: resource_binding_definition.name.to_string(),
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
                            &self.world.server.device,
                            &self.world.server.queue,
                            &mut self.world.texture_cache,
                        )?))
                    } else {
                        let target_kind = resource_binding.get_binding_type_kind();

                        Err(MaterialError::ResourceBindingDefinitionNotMatch {
                            name: resource_binding_definition.name.to_string(),
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

    pub fn process(
        &mut self,
        effect: &MaterialEffect,
    ) -> Result<MaterialBindGroupHandle, FrameworkError> {
        let mut material_resource_handles = vec![];
        let mut entries = vec![];

        for definition in effect.resource_binding_definitions.iter() {
            material_resource_handles.push(self.extra(definition)?);
            entries.push(definition.entry.clone());
        }

        let desc = BindGroupLayoutDescriptor { entries };

        let layout = self
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
