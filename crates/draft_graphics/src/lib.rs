mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    DepthStencilState,MultisampleState,
    BufferAddress, CommandBuffer, RenderPipeline, RenderPipelineDescriptor, Surface,
    SurfaceConfiguration, SurfaceTexture, VertexStepMode, util::BufferInitDescriptor,PrimitiveState
};

pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
