use draft_graphics::wgpu::SurfaceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Material is invalid. Summary: {0}")]
    MaterialInvalid(String),
    #[error("Geometry is invalid. Summary: {0}")]
    GeometryInvalid(String),
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),
}
