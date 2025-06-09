use std::sync::Arc;

use fyrox_core::{reflect::*, sparse::AtomicIndex, visitor::*};

use crate::BindGroupLayoutEntry;

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
