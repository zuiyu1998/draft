use draft_graphics::wgpu::SurfaceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),
}
