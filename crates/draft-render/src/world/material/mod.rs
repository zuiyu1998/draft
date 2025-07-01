pub mod storage;

pub use storage::*;

use std::{error::Error, path::Path, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

use crate::{
    ShaderResource,
    gfx_base::{
        BindGroupLayoutEntry, ColorTargetState, DepthStencilState, MultisampleState,
        PipelineCompilationOptions, PrimitiveState, VertexBufferLayout,
    },
};

use fyrox_resource::{Resource, ResourceData};

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct VertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub compilation_options: PipelineCompilationOptions,
    pub buffers: Vec<VertexBufferLayout>,
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct FragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub compilation_options: PipelineCompilationOptions,
    pub targets: Vec<Option<ColorTargetState>>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayoutDescriptor>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub layout: PipelineLayoutDescriptor,
    pub vertex: VertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ComputePipelineDescriptor {}

#[derive(Debug, Clone, Reflect, Visit)]
pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
    ComputePipelineDescriptor(Box<ComputePipelineDescriptor>),
}

impl PipelineDescriptor {
    pub fn render_pipeline_descriptor(&mut self) -> Option<&mut RenderPipelineDescriptor> {
        match self {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => Some(desc),
            _ => None,
        }
    }
}

impl Default for PipelineDescriptor {
    fn default() -> Self {
        PipelineDescriptor::RenderPipelineDescriptor(Box::default())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3485bce7-7b74-4970-9bf0-2b4a897b06dd")]
pub struct Material {
    pub desc: PipelineDescriptor,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("Material", &mut visitor)?;
        visitor.save_binary_to_file(path)?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
