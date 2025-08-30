mod binding;
mod effect;
mod handle;

pub use binding::*;
pub use effect::*;
pub use handle::*;

use crate::{BindGroupLayout, PipelineInfo, gfx_base::BindingTypeKind};
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path};
use thiserror::Error;

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Error)]
pub enum MaterialError {
    #[error("ResourceBindingDefinition not found. name: {name}")]
    ResourceBindingDefinitionNotFound { name: String },
    #[error("ResourceBindingDefinition not match. name: {name}, source: {source_kind}")]
    ResourceBindingDefinitionNotMatch {
        name: String,
        target_kind: BindingTypeKind,
        source_kind: BindingTypeKind,
    },
}

pub struct MaterialBindGroupHandle {
    pub bind_group_layout: BindGroupLayout,
    pub material_resource_handle_container: MaterialResourceHandleContainer,
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct Material {
    pub pipeline_info: PipelineInfo,
    effect_instances: Vec<MaterialEffectInstance>,
}

impl Material {
    pub fn effects(&self) -> &[MaterialEffectInstance] {
        &self.effect_instances
    }

    pub fn effect_mut(
        &mut self,
        effect_name: &ImmutableString,
    ) -> Option<&mut MaterialEffectInstance> {
        let position = self
            .effect_instances
            .iter()
            .position(|effect| effect.effect_name == *effect_name);
        position.map(|position| &mut self.effect_instances[position])
    }

    pub fn from_material<T: ErasedMaterial>() -> Self {
        let info = T::material_info();
        let container = MaterialEffectContainer::get_singleton();

        let mut material = Material::default();

        material.initialize(&info, &container);

        material
    }

    pub fn initialize(&mut self, info: &MaterialInfo, container: &MaterialEffectContainer) {
        let mut effect_instances = vec![];

        for effect_info in info.effect_infos.iter() {
            effect_instances.push(MaterialEffectInstance::new(effect_info, container));
        }

        self.pipeline_info = info.pipeline_info.clone();
        self.effect_instances = effect_instances;
    }

    pub fn new(pipeline_info: PipelineInfo, effect_instances: Vec<MaterialEffectInstance>) -> Self {
        Self {
            pipeline_info,
            effect_instances,
        }
    }
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("Material", &mut visitor)?;
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

pub struct MaterialInfo {
    pub pipeline_info: PipelineInfo,
    pub effect_infos: Vec<MaterialEffectInfo>,
}

pub trait ErasedMaterial {
    fn material_info() -> MaterialInfo;
}
