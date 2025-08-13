use fyrox_resource::ResourceData;
use std::{error::Error, fmt::Debug, path::Path};

use super::{FragmentState, VertexState};
use crate::gfx_base::{DepthStencilState, MultisampleState, PrimitiveState};
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use strum_macros::{AsRefStr, EnumString, VariantNames};

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct RenderPipelineInfo {
    pub label: ImmutableString,
    pub vertex: VertexState,
    pub fragment: Option<FragmentState>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Hash, Eq)]
pub struct ComputePipelineInfo {}

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
pub enum PipelineInfo {
    RenderPipelineInfo(Box<RenderPipelineInfo>),
    ComputePipelineInfo(Box<ComputePipelineInfo>),
}

impl Default for PipelineInfo {
    fn default() -> Self {
        PipelineInfo::RenderPipelineInfo(Box::default())
    }
}

impl ResourceData for PipelineInfo {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("PipelineInfo", &mut visitor)?;
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
