use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::{
    BindGroupLayoutNameContainer, FrameworkError, MaterialResourceBinding, RenderWorld,
    gfx_base::RawSampler, render_resource::RenderTexture,
};

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

pub struct MaterialPropertyGroupHandle {}

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
        name_container: &BindGroupLayoutNameContainer,
        resource_bindings: &FxHashMap<ImmutableString, MaterialResourceBinding>,
        world: &mut RenderWorld,
    ) -> Result<Self, FrameworkError> {
        let mut binding = 0;

        let mut target = vec![];

        for name in name_container.names.iter() {
            match resource_bindings.get(name).unwrap() {
                MaterialResourceBinding::Texture(v) => {
                    let resource = v.value.clone().unwrap();
                    let texture_data = world.get_or_create_texture(&resource)?;

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
                MaterialResourceBinding::PropertyGroup(_v) => {
                    todo!()
                }
            }
        }

        Ok(MaterialResourceHandleContainer(target))
    }
}
