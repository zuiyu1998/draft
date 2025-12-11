mod window;

pub use window::*;

pub struct RenderDataBundle {
    pub windows: RenderWindows,
}

impl RenderDataBundle {
    pub fn new(windows: RenderWindows) -> Self {
        RenderDataBundle { windows }
    }
}
