mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BufferAddress, CommandBuffer, DepthStencilState, MultisampleState, PrimitiveState,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource,
    Surface, SurfaceConfiguration, SurfaceTexture, VertexStepMode, util::BufferInitDescriptor,
};

pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
