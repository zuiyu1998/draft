pub mod frame_graph;
pub mod render_pipeline;
pub mod render_resource;
pub mod render_server;
pub mod renderer;

pub use wgpu;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Custom error is {0}")]
    Custom(String),
}
