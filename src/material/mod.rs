use std::sync::Arc;

use fyrox_core::{reflect::*, sparse::AtomicIndex, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct RenderPipelineDescriptor {}

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
