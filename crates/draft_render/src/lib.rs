pub mod render_resource;
pub mod render_server;

use thiserror::Error;

use crate::render_server::RenderServer;

#[derive(Debug, Error)]
pub enum FrameworkError {}

pub struct WorldRenderer {
    pub render_server: RenderServer,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer) -> Self {
        Self { render_server }
    }
}
