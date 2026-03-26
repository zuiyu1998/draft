use draft_graphics::VertexFormat;
use fyrox_core::{reflect::*, visitor::*};

#[derive(Debug, Reflect, Visit, Clone)]
pub(crate) struct MeshAttributeData {
    pub(crate) attribute: MeshVertexAttribute,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Visit)]
pub struct MeshVertexAttribute {
    /// The _unique_ id of the vertex attribute. This will also determine sort ordering
    /// when generating vertex buffers. Built-in / standard attributes will use "close to zero"
    /// indices. When in doubt, use a random / very large u64 to avoid conflicts.
    pub id: MeshVertexAttributeId,

    /// The format of the vertex attribute.
    pub format: VertexFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Reflect, Visit)]
pub struct MeshVertexAttributeId(u64);
