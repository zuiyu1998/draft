pub mod loader;

pub use loader::*;

use fyrox_resource::{ResourceData, io::ResourceIo};
use ron::ser::PrettyConfig;
use std::{error::Error, fs::File, io::Write, path::Path};
use thiserror::Error;

use fyrox_core::{TypeUuidProvider, Uuid, io::FileError, reflect::*, uuid, visitor::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum ShaderStage {
    #[default]
    Vertex,
    Fragment,
    Compute,
    Task,
    Mesh,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default, TypeUuidProvider)]
#[type_uuid(id = "0fb84fee-a2d2-4cb5-9aa3-98d3d30679c1")]
pub struct Shader {
    pub path: String,
    pub source: Source,
}

#[derive(Debug, Error)]
pub enum ShaderError {
    /// An i/o error has occurred.
    #[error("A file load error has occurred {0:?}")]
    Io(FileError),

    /// A parsing error has occurred.
    #[error("A parsing error has occurred {0:?}")]
    ParseError(#[from] ron::error::SpannedError),

    /// Bytes does not represent Utf8-encoded string.
    #[error("Bytes does not represent Utf8-encoded string.")]
    NotUtf8Source,
}

impl From<FileError> for ShaderError {
    fn from(value: FileError) -> Self {
        Self::Io(value)
    }
}

impl ResourceData for Shader {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        file.write_all(
            ron::ser::to_string_pretty(&self.source, PrettyConfig::default())?.as_bytes(),
        )?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
pub enum Source {
    Wgsl(String),
    Glsl(String, ShaderStage),
}

impl Default for Source {
    fn default() -> Self {
        Self::Wgsl("".into())
    }
}

impl Shader {
    /// Creates a shader from file.
    pub async fn from_file<P: AsRef<Path>>(
        path: P,
        io: &dyn ResourceIo,
    ) -> Result<Self, ShaderError> {
        let bytes = String::from_utf8(io.load_file(path.as_ref()).await?)
            .map_err(|_| ShaderError::NotUtf8Source)?;

        let ext = path.as_ref().file_name().unwrap().to_str().unwrap();

        let path = path.as_ref().to_str().unwrap().to_string();
        let path = path.replace(std::path::MAIN_SEPARATOR, "/");

        let shader = match ext {
            "wgsl" => Shader::from_wgsl(bytes, path),
            "vert" => Shader::from_glsl(bytes, ShaderStage::Vertex, path),
            "frag" => Shader::from_glsl(bytes, ShaderStage::Fragment, path),
            _ => panic!("unhandled extension: {ext}"),
        };
        Ok(shader)
    }

    pub fn from_wgsl(source: impl Into<String>, path: impl Into<String>) -> Shader {
        let source = source.into();
        let path = path.into();
        Shader {
            path,
            source: Source::Wgsl(source),
        }
    }

    pub fn from_glsl(
        source: impl Into<String>,
        stage: ShaderStage,
        path: impl Into<String>,
    ) -> Shader {
        let source = source.into();
        let path = path.into();
        Shader {
            path,
            source: Source::Glsl(source, stage),
        }
    }
}
