mod common;
mod render_server;

pub use common::*;
pub use render_server::*;
pub use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendState,
    Buffer, BufferAddress, BufferBinding, BufferBindingType, BufferDescriptor, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandBuffer, DepthStencilState, FragmentState, IndexFormat,
    MultisampleState, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat,
    TextureView, VertexBufferLayout, VertexState, VertexStepMode, util::BufferInitDescriptor,
};

#[derive(Clone)]
pub enum Pipeline {
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}
