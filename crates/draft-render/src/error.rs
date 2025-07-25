use std::path::PathBuf;

use thiserror::Error;

use fyrox_resource::{
    Resource, TypedResourceData,
    state::{LoadError, ResourceState},
};

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    ProcessShaderError(Box<naga_oil::compose::ComposerError>),
    #[error("Resource not loaded, path: {path:?}, error: {error:?}")]
    ResourceNotLoaded { path: PathBuf, error: LoadError },
}

impl From<naga_oil::compose::ComposerError> for FrameworkError {
    fn from(value: naga_oil::compose::ComposerError) -> Self {
        FrameworkError::ProcessShaderError(Box::new(value))
    }
}

impl<T: TypedResourceData> From<Resource<T>> for FrameworkError {
    fn from(value: Resource<T>) -> Self {
        let shader_state = value.header();

        if let ResourceState::LoadError { path, error } = &shader_state.state {
            FrameworkError::ResourceNotLoaded {
                path: path.clone(),
                error: error.clone(),
            }
        } else {
            unimplemented!()
        }
    }
}
