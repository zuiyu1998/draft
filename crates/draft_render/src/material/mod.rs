mod bind_group;

mod effect;

pub use bind_group::*;
pub use effect::*;

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, path::Path};

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct Material {
    pub name: String,
    pub info: MaterialInfo,
}

impl TypeUuidProvider for Material {
    fn type_uuid() -> Uuid {
        uuid!("0e54fe44-0c58-4108-a681-d6eefc88c234")
    }
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("Material", &mut visitor)?;
        visitor.save_ascii_to_file(path)?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

impl Material {
    pub fn new<M: IMaterial>() -> Self {
        Self {
            name: M::name(),
            info: M::material_info(),
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialInfo {
    pub effect_name: Option<String>,
    pub technique: usize,
}

pub trait IMaterial {
    fn name() -> String;

    fn material_info() -> MaterialInfo;
}
