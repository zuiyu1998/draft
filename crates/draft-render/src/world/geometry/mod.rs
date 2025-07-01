mod buffer;
mod index;
mod storage;
mod vertex;

pub use buffer::*;
use fyrox_resource::{Resource, ResourceData};
pub use index::*;
pub use storage::*;
pub use vertex::*;

use std::{error::Error, path::Path, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

pub type GeometryResource = Resource<Geometry>;

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "2c7b56fb-ce99-4830-acd6-d9937fa4c8a1")]
pub struct Geometry {
    pub vertex: Vertex,
    pub index: Index,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Geometry {
    pub fn new(vertex: Vertex, index: Index) -> Self {
        Self {
            vertex,
            cache_index: Arc::new(AtomicIndex::unassigned()),
            index,
        }
    }
}

impl ResourceData for Geometry {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, _path: &Path) -> Result<(), Box<dyn Error>> {
        //todo
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
