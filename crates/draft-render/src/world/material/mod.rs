mod binding;
mod handle;

pub use binding::*;
pub use handle::*;

use crate::{BindGroupLayout, PipelineSpecializerResource};
use fxhash::FxHashMap;
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
    specializer: PipelineSpecializerResource,
    resource_bindings: FxHashMap<ImmutableString, MaterialResourceBinding>,
}

impl Material {
    pub fn from_specializer(specializer: PipelineSpecializerResource) -> Self {
        Material::new(specializer, Default::default())
    }

    pub fn specializer(&self) -> &PipelineSpecializerResource {
        &self.specializer
    }

    pub fn resource_bindings(&self) -> &FxHashMap<ImmutableString, MaterialResourceBinding> {
        &self.resource_bindings
    }

    pub fn insert(&mut self, key: ImmutableString, binding: MaterialResourceBinding) {
        self.resource_bindings.insert(key, binding);
    }

    pub fn new(
        specializer: PipelineSpecializerResource,
        resource_bindings: FxHashMap<ImmutableString, MaterialResourceBinding>,
    ) -> Self {
        Self {
            specializer,
            resource_bindings,
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
