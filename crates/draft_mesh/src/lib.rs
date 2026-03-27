mod vertex;

pub use vertex::*;

use draft_graphics::{PrimitiveTopology, VertexFormat};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::ResourceData;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MeshAccessError {}

#[derive(Debug, Reflect, Visit, Clone)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[derive(Debug, Reflect, Clone)]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,
    attributes: HashMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,
}

impl TypeUuidProvider for Mesh {
    fn type_uuid() -> Uuid {
        uuid!("3930d4ce-f524-4420-b8c8-459b5e427e93")
    }
}

impl Mesh {
    pub fn new(primitive_topology: PrimitiveTopology) -> Self {
        Mesh {
            primitive_topology,
            attributes: Default::default(),
            indices: Default::default(),
        }
    }

    pub fn insert_attribute(
        &mut self,
        attribute: MeshVertexAttribute,
        values: impl Into<VertexAttributeValues>,
    ) {
        let values = values.into();
        let values_format = VertexFormat::from(&values);
        if values_format != attribute.format {
            panic!(
                "Failed to insert attribute. Given format is {values_format:?} but expected {:?}",
                attribute.format
            );
        }

        self.attributes
            .insert(attribute.id, MeshAttributeData { attribute, values });
    }
}

impl Visit for Mesh {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        Ok(())
    }
}

impl ResourceData for Mesh {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
