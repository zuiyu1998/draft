use std::{collections::HashMap, error::Error, path::Path};

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};

use crate::PassData;

pub type MaterialResource = Resource<Material>;

pub struct MaterialData {
    pub pass_data: PassData,
}

#[derive(Default)]
pub struct MaterialCache(HashMap<u64, MaterialData>);

impl MaterialCache {
    pub fn get(&self, material: &MaterialResource) -> Option<&MaterialData> {
        self.0.get(&material.key())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct Material {}

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
