mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BlendState, Buffer, BufferAddress,
    BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites, CommandBuffer,
    DepthStencilState, FragmentState, IndexFormat, MultisampleState, PipelineCompilationOptions,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor,
    ShaderSource, Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureView,
    VertexBufferLayout, VertexState, VertexStepMode, util::BufferInitDescriptor,BufferBinding
};

#[derive(Clone)]
pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
