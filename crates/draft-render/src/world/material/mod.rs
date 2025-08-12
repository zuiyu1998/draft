mod binding;
mod effect;
mod handle;

pub use binding::*;
pub use effect::*;
use fxhash::FxHashMap;
pub use handle::*;

use crate::{BindGroupLayout, PipelineDescriptorResource};
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path};

pub type MaterialResource = Resource<Material>;

pub struct MaterialBindGroupHandle {
    pub bind_group_layout: BindGroupLayout,
    pub material_resource_handle_container: MaterialResourceHandleContainer,
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct Material {
    pipeline_descriptor: PipelineDescriptorResource,
    effects: FxHashMap<ImmutableString, MaterialEffect>,
}

impl Material {
    pub fn from_specializer(pipeline_descriptor: PipelineDescriptorResource) -> Self {
        Material::new(pipeline_descriptor, Default::default())
    }

    pub fn pipeline_descriptor(&self) -> &PipelineDescriptorResource {
        &self.pipeline_descriptor
    }

    pub fn effect(&self, name: &ImmutableString) -> Option<&MaterialEffect> {
        self.effects.get(name)
    }

    pub fn effect_mut(&mut self, name: &ImmutableString) -> Option<&mut MaterialEffect> {
        self.effects.get_mut(name)
    }

    pub fn new(
        pipeline_descriptor: PipelineDescriptorResource,
        effects: FxHashMap<ImmutableString, MaterialEffect>,
    ) -> Self {
        Self {
            pipeline_descriptor,
            effects,
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
