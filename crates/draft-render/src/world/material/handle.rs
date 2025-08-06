use std::ops::{Deref, DerefMut};

use crate::{
    FrameworkError, RenderWorld, ResourceKey, ResourceKeyContainer, gfx_base::RawSampler,
    render_resource::RenderTexture,
};

use super::{MaterialResourceBinding, ResourceBindings};

pub enum MaterialResourceHandle {
    Texture(MaterialTextureHandle),
    Sampler(MaterialSamplerHandle),
    PropertyGroup(MaterialPropertyGroupHandle),
}

pub struct MaterialTextureHandle {
    pub binding: u32,
    pub texture: RenderTexture,
}

pub struct MaterialSamplerHandle {
    pub binding: u32,
    pub sampler: RawSampler,
}

pub struct MaterialPropertyGroupHandle {
    pub offset: u32,
    pub resource_key: ResourceKey,
}

pub struct MaterialResourceHandleContainer(Vec<MaterialResourceHandle>);

impl Deref for MaterialResourceHandleContainer {
    type Target = Vec<MaterialResourceHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialResourceHandleContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MaterialResourceHandleContainer {
    pub fn extra(
        key_container: &ResourceKeyContainer,
        resource_bindings: &ResourceBindings,
        render_world: &mut RenderWorld,
    ) -> Result<Self, FrameworkError> {
        let mut binding = 0;

        let mut target = vec![];

        for key in key_container.keys.iter() {
            match resource_bindings.get(key).unwrap() {
                MaterialResourceBinding::Texture(v) => {
                    let resource = v.value.clone().unwrap();
                    let texture_data = render_world.get_or_create_texture(&resource)?;

                    target.push(MaterialResourceHandle::Texture(MaterialTextureHandle {
                        binding,
                        texture: texture_data.render_data.texture.clone(),
                    }));

                    binding += 1;

                    target.push(MaterialResourceHandle::Sampler(MaterialSamplerHandle {
                        binding,
                        sampler: texture_data.render_data.sampler.sampler().clone(),
                    }));

                    binding += 1;
                }
                MaterialResourceBinding::PropertyGroup(v) => {
                    let named_values_container = v.get_named_values_container();
                    let offset = render_world
                        .buffer_allocator
                        .write(key, named_values_container);

                    target.push(MaterialResourceHandle::PropertyGroup(
                        MaterialPropertyGroupHandle {
                            offset,
                            resource_key: key.clone(),
                        },
                    ));

                    binding += 1;
                }
            }
        }

        Ok(MaterialResourceHandleContainer(target))
    }
}
