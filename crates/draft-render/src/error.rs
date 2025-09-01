use std::path::PathBuf;

use fyrox_core::{io::FileError, visitor::error::VisitError};
use thiserror::Error;
use toml::de::Error as TomlError;

use fyrox_resource::{
    Resource, TypedResourceData,
    state::{LoadError, ResourceState},
};

use crate::MaterialError;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    ProcessShaderError(Box<naga_oil::compose::ComposerError>),
    #[error("Resource not loaded, path: {path:?}, error: {error:?}")]
    ResourceNotLoaded { path: PathBuf, error: LoadError },
    #[error("Resource is loading, path: {path:?}")]
    ResourcePending { path: PathBuf },
    #[error(transparent)]
    MaterialError(#[from] MaterialError),
    #[error(transparent)]
    Visit(#[from] VisitError),
    #[error("file error: {0:?}")]
    FileError(FileError),
    #[error(transparent)]
    TomlError(#[from] TomlError),
}

impl From<FileError> for FrameworkError {
    fn from(value: FileError) -> Self {
        FrameworkError::FileError(value)
    }
}

impl From<naga_oil::compose::ComposerError> for FrameworkError {
    fn from(value: naga_oil::compose::ComposerError) -> Self {
        FrameworkError::ProcessShaderError(Box::new(value))
    }
}

impl<T: TypedResourceData> From<Resource<T>> for FrameworkError {
    fn from(value: Resource<T>) -> Self {
        let resource_state = value.header();

        match &resource_state.state {
            ResourceState::LoadError { path, error } => FrameworkError::ResourceNotLoaded {
                path: path.clone(),
                error: error.clone(),
            },
            ResourceState::Pending { path, .. } => {
                FrameworkError::ResourcePending { path: path.clone() }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
