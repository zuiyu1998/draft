use draft_core::pool::Handle;
use draft_window::SystemWindow;

pub struct RenderCamera {
    pub render_target: RenderTarget,
}

impl RenderCamera {
    pub fn new(render_target: RenderTarget) -> Self {
        Self { render_target }
    }

    pub fn primary() -> Self {
        Self {
            render_target: RenderTarget::Primary,
        }
    }
}

#[derive(Clone, Default)]
pub enum RenderTarget {
    #[default]
    Primary,
    Window(Handle<SystemWindow>),
}
