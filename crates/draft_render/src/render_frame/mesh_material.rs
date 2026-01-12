use draft_material::{MaterialClass, MaterialResource};

use draft_mesh::{MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts};
use fxhash::FxHashMap;
use std::{
    collections::hash_map::Entry,
    ops::{Deref, DerefMut},
};

use crate::{BatchRenderMeshMaterial, CachedRenderPipelineId, MeshMaterialPipeline, PipelineCache};

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

pub struct BatchMeshMaterial {
    pub mesh: MeshResource,
    pub material: MaterialResource,
    pub instance: MeshMaterialInstanceData,
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

#[derive(Default)]
pub struct BatchMeshMaterialContainer(FxHashMap<BatchMeshMaterialKey, Vec<BatchMeshMaterial>>);

impl Deref for BatchMeshMaterialContainer {
    type Target = FxHashMap<BatchMeshMaterialKey, Vec<BatchMeshMaterial>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BatchMeshMaterialContainer {
    pub fn push(
        &mut self,
        mesh: MeshResource,
        material: MaterialResource,
        instance: MeshMaterialInstanceData,
        layouts: &mut MeshVertexBufferLayouts,
        mesh_material_pipeline: &mut MeshMaterialPipeline,
        pipeline_cache: &mut PipelineCache,
    ) {
        let pipeline_id = mesh_material_pipeline.get(&mesh, &material, pipeline_cache, layouts);
        let key = BatchMeshMaterialKey::new(&mesh, &material, layouts, pipeline_id);

        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(BatchMeshMaterial {
                    mesh,
                    material,
                    instance,
                });
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![BatchMeshMaterial {
                    mesh,
                    material,
                    instance,
                }]);
            }
        }
    }
}
