use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
}
