use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("mesh not loaded.")]
    MeshNotLoaded,
}
