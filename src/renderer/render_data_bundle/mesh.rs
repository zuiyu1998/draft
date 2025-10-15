use std::hash::Hasher;

use draft_render::{
    GeometryResource, MaterialResource,
    gfx_base::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode},
};
use fxhash::{FxHashMap, FxHasher};
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
        }
    }
}

#[derive(Clone)]
pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instance_data: Vec<MeshInstanceData>,
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
        geometry: GeometryResource,
        material: MaterialResource,
        _sort_index: u64,
        instance_data: MeshInstanceData,
    ) {
        let mut hashser = FxHasher::default();
        hashser.write_u64(geometry.key());
        hashser.write_u64(material.key());
        let key = hashser.finish();

        self.batches
            .entry(key)
            .or_insert_with(|| Batch::new(geometry, material))
            .instance_data
            .push(instance_data);
    }
}
