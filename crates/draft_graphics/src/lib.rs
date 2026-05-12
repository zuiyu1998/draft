mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BufferAddress, CommandBuffer, Surface, SurfaceTexture, VertexStepMode,
    util::BufferInitDescriptor,SurfaceConfiguration
};

pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
