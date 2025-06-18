pub mod vertex;

pub use vertex::*;

use std::sync::Arc;

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "2c7b56fb-ce99-4830-acd6-d9937fa4c8a1")]
pub struct Geometry {
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}
