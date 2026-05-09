pub mod frame_graph;

use draft_graphics::RenderServer;
use draft_window::SystemWindowManager;

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_server,
            system_window_manager,
        }
    }

    pub fn render(&mut self) {}
}
