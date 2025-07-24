use crate::{
    BindGroupLayoutDescriptor, PipelineLayoutDescriptor, ShaderResource,
    gfx_base::{
        ColorTargetState, DepthStencilState, MultisampleState, PrimitiveState, VertexBufferLayout,
    },
};

use fyrox_core::{ImmutableString, reflect::*, visitor::*};

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
    pub fn insert_bind_group_layout(
        &mut self,
        key: ImmutableString,
        value: BindGroupLayoutDescriptor,
    ) {
        self.layout.insert(key, value);
    }

    pub fn merge(&mut self, other: &RenderPipelineDescriptor) {
        self.label = other.label.clone();
        for (name, value) in other.layout.iter() {
            self.layout.insert(name.clone(), value.clone());
        }
        self.vertex = other.vertex.clone();
        self.primitive = other.primitive;
        self.depth_stencil = other.depth_stencil.clone();
        self.multisample = other.multisample;
        self.fragment = other.fragment.clone();
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Hash, Eq)]
pub struct ComputePipelineDescriptor {}

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
