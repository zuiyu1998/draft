mod vertex;
mod buffer;

pub use vertex::*;
pub use buffer::*;

use draft_graphics::PrimitiveTopology;
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::ResourceData;
use std::collections::HashMap;

#[derive(Debug, Reflect, Visit, Clone)]
pub(crate) struct Indices {}

#[derive(Debug, Reflect, Clone)]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,
    attributes: HashMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Indices,
}

impl TypeUuidProvider for Mesh {
    fn type_uuid() -> Uuid {
        uuid!("3930d4ce-f524-4420-b8c8-459b5e427e93")
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
