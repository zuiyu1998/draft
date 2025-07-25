use crate::{
    ShaderResource,
    gfx_base::{ColorTargetState, VertexBufferLayout},
};

use fyrox_core::{reflect::*, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Hash, Eq)]
pub struct VertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub buffers: Vec<VertexBufferLayout>,
}

#[derive(Clone, Debug, Reflect, Visit, Default, PartialEq, Hash, Eq)]
pub struct FragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub targets: Vec<Option<ColorTargetState>>,
}
