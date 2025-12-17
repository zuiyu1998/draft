use std::collections::HashMap;

use draft_graphics::gfx_base::{GpuSurfaceTexture, TextureView};

pub struct RenderWindow {
    pub surface_texture: GpuSurfaceTexture,
    pub surface_texture_view: TextureView,
}

#[derive(Default)]
pub struct RenderWindows {
    primary: Option<usize>,
    data: HashMap<usize, RenderWindow>,
}

impl RenderWindows {
    pub fn into_iter(self) -> impl Iterator<Item = RenderWindow> {
        self.data.into_values()
    }

    pub fn primary(&self) -> Option<&RenderWindow> {
        self.primary.as_ref().and_then(|id| self.data.get(id))
    }

    pub fn set_primary(&mut self, index: usize) {
        self.primary = Some(index)
    }

    pub fn insert(&mut self, index: usize, window: RenderWindow) {
        self.data.insert(index, window);
    }
}
