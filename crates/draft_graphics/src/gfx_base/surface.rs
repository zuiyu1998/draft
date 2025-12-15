use wgpu::SurfaceError;

use crate::gfx_base::GpuSurfaceTexture;

pub struct GpuSurface(wgpu::Surface<'static>);

impl GpuSurface {
    pub(crate) fn new(surface: wgpu::Surface<'static>) -> Self {
        Self(surface)
    }

    pub(crate) fn get_wgpu_surface(&self) -> &wgpu::Surface<'static> {
        &self.0
    }

    pub fn get_current_texture(&self) -> Result<GpuSurfaceTexture, SurfaceError> {
        let texture = self.0.get_current_texture()?;

        Ok(GpuSurfaceTexture::new(texture))
    }
}
