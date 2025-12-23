use draft_material::{MaterialClass, MaterialResource};

use draft_geometry::{
    GeometryResource, GeometryVertexBufferLayoutRef, GeometryVertexBufferLayouts,
};
use fxhash::FxHashMap;
use std::{collections::hash_map::Entry, ops::Deref};

pub struct GeometryInstanceData {}

pub struct BatchMesh {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instance: GeometryInstanceData,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BatchMeshKey {
    pub geometry_layout: GeometryVertexBufferLayoutRef,
    pub material_class: MaterialClass,
}

impl BatchMeshKey {
    pub fn new(
        geometry: &GeometryResource,
        material: &MaterialResource,
        layouts: &mut GeometryVertexBufferLayouts,
    ) -> BatchMeshKey {
        let geometry = geometry.data_ref();
        let geometry_layout = geometry.get_geometry_vertex_buffer_layout(layouts);

        let material = material.data_ref();
        let material_class = material.get_class();

        BatchMeshKey {
            geometry_layout,
            material_class,
        }
    }
}

#[derive(Default)]
pub struct BatchMeshContainer(FxHashMap<BatchMeshKey, Vec<BatchMesh>>);

impl Deref for BatchMeshContainer {
    type Target = FxHashMap<BatchMeshKey, Vec<BatchMesh>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BatchMeshContainer {
    pub fn push(
        &mut self,
        geometry: GeometryResource,
        material: MaterialResource,
        instance: GeometryInstanceData,
        layouts: &mut GeometryVertexBufferLayouts,
    ) {
        let key = BatchMeshKey::new(&geometry, &material, layouts);

        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                entry.into_mut().push(BatchMesh {
                    geometry,
                    material,
                    instance,
                });
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![BatchMesh {
                    geometry,
                    material,
                    instance,
                }]);
            }
        }
    }
}
