use std::path::PathBuf;

use thiserror::Error;

use crate::ShaderResource;
use fyrox_resource::state::{LoadError, ResourceState};

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    ProcessShaderError(Box<naga_oil::compose::ComposerError>),
    #[error("Shader not loaded, path: {path:?}, error: {error:?}")]
    ShaderNotLoaded { path: PathBuf, error: LoadError },
}

impl From<naga_oil::compose::ComposerError> for FrameworkError {
    fn from(value: naga_oil::compose::ComposerError) -> Self {
        FrameworkError::ProcessShaderError(Box::new(value))
    }
}

impl From<ShaderResource> for FrameworkError {
    fn from(value: ShaderResource) -> Self {
        let shader_state = value.header();

        if let ResourceState::LoadError { path, error } = &shader_state.state {
            FrameworkError::ShaderNotLoaded {
                path: path.clone(),
                error: error.clone(),
            }
        } else {
            unimplemented!()
        }
    }
}
