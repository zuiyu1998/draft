pub mod render_resource;
pub mod render_server;

pub use wgpu;

use draft_window::SystemWindowManager;
use thiserror::Error;

use crate::render_server::RenderServer;

#[derive(Debug, Error)]
pub enum FrameworkError {}

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
}
