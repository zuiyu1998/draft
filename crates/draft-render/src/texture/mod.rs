use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::frame_graph::TextureInfo;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct SamplerInfo {}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "8ebc2e08-a5ae-4fd0-9ef7-6882d73ac871")]
pub struct Texture {
    bytes: TextureBytes,
    sampler_info: SamplerInfo,
    texture_info: TextureInfo,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}

#[derive(Default, Clone, Reflect)]
pub struct TextureBytes(Vec<u8>);

impl Visit for TextureBytes {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.0.visit(name, visitor)
    }
}

impl Debug for TextureBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture has {} bytes", self.0.len())
    }
}

impl From<Vec<u8>> for TextureBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Deref for TextureBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
