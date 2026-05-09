mod index;
mod vertex;

pub use index::*;
pub use vertex::*;

use std::sync::Arc;

use draft_graphics::PrimitiveTopology;
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid};

#[derive(Debug, Clone, Default, Reflect, TypeUuidProvider)]
#[type_uuid(id = "8a23a414-e66d-4e12-9628-92c6ab49c2f0")]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,

    pub vertex_buffer: VertexBuffer,
    pub index_buffer: Option<IndexBuffer>,

    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}
