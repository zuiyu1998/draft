mod window;

pub use window::*;

#[derive(Default)]
pub struct RenderDataBundle {
    pub windows: RenderWindows,
}

impl RenderDataBundle {
    pub fn empty() -> Self {
        RenderDataBundle::default()
    }
}
