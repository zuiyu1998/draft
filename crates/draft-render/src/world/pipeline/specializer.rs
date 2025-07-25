use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path};

use super::{PipelineDescriptor, RenderPipelineDescriptor};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};

pub type PipelineSpecializerResource = Resource<PipelineSpecializer>;

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "b4c3e37b-5150-4228-a7fb-c29b07a03e2f")]
pub struct PipelineSpecializer {
    pub desc: PipelineDescriptor,
}

impl PipelineSpecializer {
    pub fn new_render_specializer(desc: RenderPipelineDescriptor) -> Self {
        PipelineSpecializer {
            desc: PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc)),
        }
    }
}

impl ResourceData for PipelineSpecializer {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("PipelineSpecializer", &mut visitor)?;
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
