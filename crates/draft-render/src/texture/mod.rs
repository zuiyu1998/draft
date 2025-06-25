mod storage;

use fyrox_resource::{Resource, ResourceData};
pub use storage::*;

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use std::{
    error::Error,
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};

use crate::{
    frame_graph::TextureInfo,
    gfx_base::{RawSamplerDescriptor, RawTextureDescriptor, SamplerInfo},
};

pub type TextureResource = Resource<Texture>;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct TextureSamplerInfo {
    info: SamplerInfo,
    #[visit(optional)]
    modifications_counter: u64,
}

impl TextureSamplerInfo {
    pub fn get_desc(&self) -> &RawSamplerDescriptor {
        todo!()
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct Image {
    pub bytes: TextureBytes,
    pub texture_info: TextureInfo,
    #[visit(optional)]
    modifications_counter: u64,
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "8ebc2e08-a5ae-4fd0-9ef7-6882d73ac871")]
pub struct Texture {
    sampler_info: TextureSamplerInfo,
    image: Image,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Texture {
    pub fn get_desc(&self) -> &RawTextureDescriptor {
        todo!()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.image.bytes
    }
}

impl ResourceData for Texture {
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
