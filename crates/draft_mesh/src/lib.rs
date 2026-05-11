mod index;
mod vertex;

use draft_graphics::PrimitiveTopology;
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};
use std::sync::Arc;

pub use index::*;
pub use vertex::*;

pub type MeshResource = Resource<Mesh>;

#[derive(Debug, Clone, Default, Reflect, TypeUuidProvider)]
#[type_uuid(id = "8a23a414-e66d-4e12-9628-92c6ab49c2f0")]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,

    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,

    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Visit for Mesh {
    fn visit(&mut self, _name: &str, _visitor: &mut Visitor) -> VisitResult {
        todo!()
    }
}

impl ResourceData for Mesh {
    fn type_uuid(&self) -> Uuid {
        <Mesh as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        false
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        todo!()
    }
}
