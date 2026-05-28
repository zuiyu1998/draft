mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BlendState, BufferAddress, Color, ColorTargetState, ColorWrites, CommandBuffer,
    DepthStencilState, FragmentState, MultisampleState, PipelineCompilationOptions, PrimitiveState,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource,
    Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureView, VertexBufferLayout,
    VertexState, VertexStepMode, util::BufferInitDescriptor,
};

#[derive(Clone)]
pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
