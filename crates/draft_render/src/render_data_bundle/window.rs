use frame_graph::gfx_base::{GpuSurfaceTexture, TextureView};

pub struct RenderWindow {
    pub surface_texture: GpuSurfaceTexture,
    pub surface_texture_view: TextureView,
}

pub struct RenderWindows(RenderWindow);

impl RenderWindows {
    pub fn primary(&self) -> &RenderWindow {
        &self.0
    }
}
