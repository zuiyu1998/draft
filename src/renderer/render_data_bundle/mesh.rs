use draft_render::{
    GeometryResource, MaterialResource,
    gfx_base::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode},
};
use fxhash::FxHashMap;
use fyrox_core::algebra::Matrix4;

use crate::scene::Node;

pub trait MeshRenderDataBundleStorage: 'static {
    fn push_mesh(
        &mut self,
        geometry: GeometryResource,
        material: MaterialResource,
        sort_index: u64,
        instance_data: MeshInstanceData,
    );
}

#[derive(Clone)]
pub struct MeshInstanceData {
    pub view_projection_matrix: Matrix4<f32>,
    pub layout: VertexBufferLayout,
}

pub fn vertex_buffer_layout() -> VertexBufferLayout {
    VertexBufferLayout {
        array_stride: 64,
        step_mode: VertexStepMode::Instance,
        attributes: vec![
            VertexAttribute {
                offset: 0,
                shader_location: 5,
                format: VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: 16,
                shader_location: 6,
                format: VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: 32,
                shader_location: 7,
                format: VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: 48,
                shader_location: 8,
                format: VertexFormat::Float32x4,
            },
        ],
    }
}

impl MeshInstanceData {
    pub fn from_node(node: &Node) -> Self {
        MeshInstanceData {
            view_projection_matrix: node.global_transform.clone().into_inner(),
            layout: vertex_buffer_layout(),
        }
    }
}

#[derive(Clone)]
pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instance_data: Vec<MaterialResource>,
}

impl Batch {
    pub fn new(geometry: GeometryResource, material: MaterialResource) -> Self {
        Self {
            geometry,
            material,
            instance_data: vec![],
        }
    }
}

#[derive(Default)]
pub struct BatchContainer {
    pub batches: FxHashMap<u64, Batch>,
}

impl MeshRenderDataBundleStorage for BatchContainer {
    fn push_mesh(
        &mut self,
        _geometry: GeometryResource,
        _material: MaterialResource,
        _sort_index: u64,
        _instance_data: MeshInstanceData,
    ) {
        todo!()
    }
}
