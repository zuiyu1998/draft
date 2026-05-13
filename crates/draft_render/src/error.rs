use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Mesh not loaded.")]
    MeshNotLoaded,
    #[error("Shader not loaded.")]
    ShaderNotLoaded,
}
