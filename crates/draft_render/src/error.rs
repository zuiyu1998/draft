use draft_graphics::wgpu::SurfaceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Material is invalid. Summary: {0}")]
    MaterialInvalid(String),
    #[error("Mesh is invalid. Summary: {0}")]
    MeshInvalid(String),
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),
}
