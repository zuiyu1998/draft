use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Mesh not loaded.")]
    MeshNotLoaded,
    #[error("Shader not loaded.")]
    ShaderNotLoaded,
    #[error("Resource not loaded. The summary is {0}")]
    ResourceNotLoaded(String),
}
