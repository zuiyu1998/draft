mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BlendState, BufferAddress, ColorTargetState, ColorWrites, CommandBuffer, DepthStencilState,
    FragmentState, MultisampleState, PipelineCompilationOptions, PrimitiveState, RenderPipeline,
    RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, Surface,
    SurfaceConfiguration, SurfaceTexture, VertexBufferLayout, VertexState, VertexStepMode,
    util::BufferInitDescriptor,TextureFormat
};

pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
