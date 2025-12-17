use std::collections::HashMap;

use draft_graphics::gfx_base::{GpuSurfaceTexture, TextureView};
use draft_window::Window;
use fyrox_resource::core::pool::Handle;

pub struct RenderWindow {
    pub surface_texture: GpuSurfaceTexture,
    pub surface_texture_view: TextureView,
}

#[derive(Default)]
pub struct RenderWindows {
    primary: Option<Handle<Window>>,
    data: HashMap<Handle<Window>, RenderWindow>,
}

impl RenderWindows {
    pub fn primary(&self) -> Option<&RenderWindow> {
        self.primary.as_ref().and_then(|id| self.data.get(id))
    }
}
