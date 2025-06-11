use thiserror::Error;

use crate::ShaderResource;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
    #[error("Shader not loaded: {0:?}")]
    ShaderNotLoaded(ShaderResource),
}
