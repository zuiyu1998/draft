mod container;
mod loader;
mod resource_bindings;

pub use container::*;
pub use loader::*;
pub use resource_bindings::*;

use std::{error::Error, fs::File, io::Write, path::Path};

use crate::{BindGroupLayout, FrameworkError, MaterialResourceHandle};
use draft_gfx_base::{BindGroupLayoutDescriptor, BindGroupLayoutEntry};
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

impl ResourceBindingDefinition {
    pub fn is_internal(&self) -> bool {
        self.name.starts_with("internal/")
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
