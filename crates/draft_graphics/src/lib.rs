mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{BufferAddress, VertexStepMode, util::BufferInitDescriptor};

pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
