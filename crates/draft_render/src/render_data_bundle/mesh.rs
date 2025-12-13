use crate::MaterialResource;
use draft_geometry::GeometryResource;
use fxhash::{FxHashMap, FxHasher};
use std::{collections::hash_map::Entry, hash::Hasher};

pub struct GeometryInstanceData {}

pub struct BatchMesh {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    pub instances: Vec<GeometryInstanceData>,
}

impl BatchMesh {
    pub fn key(geometry: &GeometryResource, material: &MaterialResource) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write_u64(geometry.key());
        hasher.write_u64(material.key());

        hasher.finish()
    }
}

#[derive(Default)]
pub struct BatchMeshContainer(FxHashMap<u64, BatchMesh>);

impl BatchMeshContainer {
    pub fn push(
        &mut self,
        geometry: GeometryResource,
        material: MaterialResource,
        instance: GeometryInstanceData,
    ) {
        let key = BatchMesh::key(&geometry, &material);

        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                entry.into_mut().instances.push(instance);
            }
            Entry::Vacant(entry) => {
                entry.insert(BatchMesh {
                    geometry,
                    material,
                    instances: vec![instance],
                });
            }
        }
    }
}
