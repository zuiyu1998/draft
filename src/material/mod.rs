use std::sync::Arc;

use fyrox_core::{reflect::*, sparse::AtomicIndex, visitor::*};

use crate::{
    BindGroupLayoutEntry, DepthStencilState, PipelineCompilationOptions, PrimitiveState,
    ShaderResource, VertexBufferLayout,
};

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct VertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub compilation_options: PipelineCompilationOptions,
    pub buffers: Vec<VertexBufferLayout>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct BindGroupLayoutDescriptor {
    pub label: String,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct PipelineLayoutDescriptor {
    pub label: String,
    pub bind_group_layouts: Vec<BindGroupLayoutDescriptor>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub layout: PipelineLayoutDescriptor,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ComputePipelineDescriptor {}

#[derive(Debug, Clone, Reflect, Visit)]
pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
    ComputePipelineDescriptor(Box<ComputePipelineDescriptor>),
}

#[derive(Debug, Clone, Reflect, Visit)]
pub struct Material {
    desc: PipelineDescriptor,
    /// An id that can be used to create associated GPU resources.
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}
