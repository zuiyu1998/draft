use draft_material::{MaterialClass, MaterialResource};

use draft_mesh::{MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts};
use fxhash::FxHashMap;
use std::ops::{Deref, DerefMut};

use crate::{BatchRenderMeshMaterial, CachedRenderPipelineId};

#[derive(Default)]
pub struct BatchRenderMeshMaterialContainer(
    FxHashMap<BatchMeshMaterialKey, Vec<BatchRenderMeshMaterial>>,
);

impl Deref for BatchRenderMeshMaterialContainer {
    type Target = FxHashMap<BatchMeshMaterialKey, Vec<BatchRenderMeshMaterial>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BatchRenderMeshMaterialContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MeshMaterialInstanceData {
    pub data: Vec<u8>,
    pub buffer_size: u64,
}

pub trait IntoMeshMaterialInstanceData {
    fn into_mesh_material_instance_data(self) -> MeshMaterialInstanceData;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BatchMeshMaterialKey {
    pub mesh_layout: MeshVertexBufferLayoutRef,
    pub material_class: MaterialClass,
    pub pipeline_id: CachedRenderPipelineId,
}

impl BatchMeshMaterialKey {
    pub fn new(
        mesh: &MeshResource,
        material: &MaterialResource,
        layouts: &mut MeshVertexBufferLayouts,
        pipeline_id: CachedRenderPipelineId,
    ) -> BatchMeshMaterialKey {
        let mesh = mesh.data_ref();
        let mesh_layout = mesh.get_mesh_vertex_buffer_layout(layouts);

        let material = material.data_ref();
        let material_class = material.get_class();

        BatchMeshMaterialKey {
            mesh_layout,
            material_class,
            pipeline_id,
        }
    }
}
