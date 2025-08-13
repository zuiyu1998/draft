mod binding;
mod effect;
mod handle;

pub use binding::*;
use draft_gfx_base::BindingTypeKind;
pub use effect::*;
pub use handle::*;

use crate::{BindGroupLayout, PipelineInfoResource};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
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
    pub pipeline_info: PipelineInfoResource,
    pub effects: Vec<MaterialEffect>,
}

impl Material {
    pub fn from_pipeline_info(pipeline_info: PipelineInfoResource) -> Self {
        Material::new(pipeline_info, Default::default())
    }

    pub fn new(pipeline_info: PipelineInfoResource, effects: Vec<MaterialEffect>) -> Self {
        Self {
            pipeline_info,
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
