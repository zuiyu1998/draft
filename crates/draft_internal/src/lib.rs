pub use draft_app as app;
pub use draft_render as render;

use draft_app::{App, Plugin};
use draft_winit::WinitPlugin;

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugin(WinitPlugin);
    }
}
