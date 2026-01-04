mod bind_group;
mod effect;
mod resource_binding;

pub use bind_group::*;
pub use effect::*;

use draft_graphics::{
    ColorTargetState, DepthStencilState, MultisampleState, PrimitiveState, PushConstantRange,
};
use draft_shader::{Shader, ShaderDefVal, ShaderResource};
use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData, manager::BuiltInResource};
use std::{error::Error, path::Path};

use crate::resource_binding::MaterialResourceBinding;

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct MaterialEffctInfo {
    pub effect_name: String,
    pub technique: usize,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialVertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub shader_defs: Vec<ShaderDefVal>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialFragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub targets: Vec<Option<ColorTargetState>>,
    pub shader_defs: Vec<ShaderDefVal>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct PipelineState {
    pub vertex: MaterialVertexState,
    pub fragment: Option<MaterialFragmentState>,
    pub push_constant_ranges: Vec<PushConstantRange>,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub primitive: PrimitiveState,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MaterialClass {
    pub effct_info: MaterialEffctInfo,
}

impl MaterialClass {
    pub fn new(effct_info: MaterialEffctInfo) -> Self {
        Self { effct_info }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct Material {
    pub effct_info: MaterialEffctInfo,
    pub pipeline_state: PipelineState,
    pub resource_bindings: FxHashMap<ImmutableString, MaterialResourceBinding>,
}

impl TypeUuidProvider for Material {
    fn type_uuid() -> Uuid {
        uuid!("0e54fe44-0c58-4108-a681-d6eefc88c234")
    }
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("Material", &mut visitor)?;
        visitor.save_ascii_to_file(path)?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

impl Material {
    pub fn new<M: IMaterial>() -> Self {
        let info = M::info();

        Self {
            effct_info: info.effct_info,
            pipeline_state: info.pipeline_state,
            resource_bindings: Default::default(),
        }
    }

    pub fn get_class(&self) -> MaterialClass {
        MaterialClass::new(self.effct_info.clone())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct MaterialInfo {
    pub effct_info: MaterialEffctInfo,
    pub pipeline_state: PipelineState,
}

pub trait IMaterial {
    fn name() -> &'static str;

    fn info() -> MaterialInfo;

    fn built_in_shaders() -> Vec<&'static BuiltInResource<Shader>> {
        vec![]
    }

    fn built_in_material_effects() -> Vec<&'static BuiltInResource<MaterialEffect>> {
        vec![]
    }
}
