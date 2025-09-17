use draft_render::gfx_base::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use fyrox_core::algebra::Matrix4;

use crate::scene::Node;

#[derive(Clone)]
pub struct MeshInstanceData {
    pub view_projection_matrix: Matrix4<f32>,
    pub layout: VertexBufferLayout,
}

fn layout() -> VertexBufferLayout {
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
            layout: layout(),
        }
    }
}
