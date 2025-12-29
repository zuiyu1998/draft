use draft_material::{MaterialClass, MaterialResource};

use draft_mesh::{
    MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts,
};
use fxhash::FxHashMap;
use std::{collections::hash_map::Entry, ops::Deref};

pub struct MeshInstanceData {}

pub struct BatchMeshMaterial {
    pub mesh: MeshResource,
    pub material: MaterialResource,
    pub instance: MeshInstanceData,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BatchMeshMaterialKey {
    pub mesh_layout: MeshVertexBufferLayoutRef,
    pub material_class: MaterialClass,
}

impl BatchMeshMaterialKey {
    pub fn new(
        mesh: &MeshResource,
        material: &MaterialResource,
        layouts: &mut MeshVertexBufferLayouts,
    ) -> BatchMeshMaterialKey {
        let mesh = mesh.data_ref();
        let mesh_layout = mesh.get_mesh_vertex_buffer_layout(layouts);

        let material = material.data_ref();
        let material_class = material.get_class();

        BatchMeshMaterialKey {
            mesh_layout,
            material_class,
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
        instance: MeshInstanceData,
        layouts: &mut MeshVertexBufferLayouts,
    ) {
        let key = BatchMeshMaterialKey::new(&mesh, &material, layouts);

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
