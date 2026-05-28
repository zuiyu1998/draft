pub use draft_app as app;
pub use draft_render as render;
pub use draft_mesh as mesh;
pub use draft_graphics as graphics;
pub use fyrox_core as core;
pub use fyrox_resource as resource;

use draft_app::{App, Plugin};
use draft_winit::WinitPlugin;

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugin(WinitPlugin);
    }
}
