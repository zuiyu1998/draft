use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path};

use super::{BindGroupLayoutDescriptor, FragmentState, PipelineLayoutDescriptor, VertexState};
use crate::gfx_base::{DepthStencilState, MultisampleState, PrimitiveState};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use strum_macros::{AsRefStr, EnumString, VariantNames};

pub type PipelineDescriptorResource = Resource<PipelineDescriptor>;

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub layout: PipelineLayoutDescriptor,
    pub vertex: VertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState>,
}

impl RenderPipelineDescriptor {
    pub fn push_bind_group_layout(&mut self, value: BindGroupLayoutDescriptor) {
        self.layout.push(value);
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Hash, Eq)]
pub struct ComputePipelineDescriptor {}

#[derive(
    Debug,
    Clone,
    Reflect,
    Visit,
    PartialEq,
    Eq,
    Hash,
    TypeUuidProvider,
    AsRefStr,
    EnumString,
    VariantNames,
)]
#[type_uuid(id = "b4c3e37b-5150-4228-a7fb-c29b07a03e2f")]
pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
    ComputePipelineDescriptor(Box<ComputePipelineDescriptor>),
}

impl PipelineDescriptor {
    pub fn new_render_specializer(desc: RenderPipelineDescriptor) -> Self {
        Self::RenderPipelineDescriptor(Box::new(desc))
    }

    pub fn render_pipeline_descriptor(&self) -> Option<&RenderPipelineDescriptor> {
        match self {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => Some(desc),
            _ => None,
        }
    }

    pub fn render_pipeline_descriptor_mut(&mut self) -> Option<&mut RenderPipelineDescriptor> {
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

impl ResourceData for PipelineDescriptor {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("PipelineDescriptor", &mut visitor)?;
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
