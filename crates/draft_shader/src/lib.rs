mod loader;

use std::{path::Path, sync::Arc};

use fyrox_core::{
    TypeUuidProvider, Uuid, io::FileError, reflect::*, sparse::AtomicIndex, uuid, visitor::*,
};

use fyrox_resource::{Resource, ResourceData, io::ResourceIo};
use thiserror::Error;

pub type ShaderResource = Resource<Shader>;

pub use loader::*;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("A file load error has occurred {0:?}")]
    Io(#[from] FileError),
}

#[derive(Debug, Clone, Default, Reflect, TypeUuidProvider)]
#[type_uuid(id = "04683000-0dc5-415b-952f-7f36c120ad0e")]
pub struct Shader {
    pub source: Source,

    #[reflect(hidden)]
    pub modifications_counter: u64,

    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Shader {
    pub async fn from_file<P: AsRef<Path>>(
        path: P,
        io: &dyn ResourceIo,
    ) -> Result<Self, ShaderError> {
        let bytes = io.load_file(path.as_ref()).await?;
        let content = String::from_utf8_lossy(&bytes);
        Ok(Self {
            source: Source::from_str(&content),
            cache_index: Default::default(),
            modifications_counter: 0,
        })
    }
}

impl ResourceData for Shader {
    fn type_uuid(&self) -> Uuid {
        <Shader as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        false
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

impl Visit for Shader {
    fn visit(&mut self, _name: &str, _visitor: &mut Visitor) -> VisitResult {
        todo!()
    }
}

#[derive(Debug, Clone, Reflect, Visit)]
pub enum Source {
    Wgsl(String),
}

impl Source {
    pub fn from_str(str: &str) -> Self {
        Source::Wgsl(str.to_string())
    }
}

impl Default for Source {
    fn default() -> Self {
        Self::Wgsl(String::new())
    }
}
