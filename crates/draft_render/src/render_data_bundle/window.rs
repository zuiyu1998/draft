use std::collections::HashMap;

use frame_graph::gfx_base::{GpuSurfaceTexture, TextureView};

pub struct RenderWindow {
    pub surface_texture: GpuSurfaceTexture,
    pub surface_texture_view: TextureView,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WindowId(usize);


#[derive(Default)]
pub struct RenderWindows {
    primary: Option<WindowId>,
    data: HashMap<WindowId, RenderWindow>,
}

impl RenderWindows {
    pub fn primary(&self) -> Option<&RenderWindow> {
        self.primary.as_ref().and_then(|id| self.data.get(id))
    }
}
