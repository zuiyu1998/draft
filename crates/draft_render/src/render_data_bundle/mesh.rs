use crate::{CachedRenderPipelineId, MaterialResource, PipelineCache};
use draft_geometry::GeometryResource;
use fxhash::{FxHashMap, FxHasher};
use std::{
    collections::{HashMap, hash_map::Entry},
    hash::Hasher,
};

pub struct SpecializedMeshPipeline {
    cache: HashMap<u64, CachedRenderPipelineId>,
}

impl SpecializedMeshPipeline {
    pub fn specialize(
        &mut self,
        batch: &BatchMesh,
        _pipeline_cache: &mut PipelineCache,
    ) -> CachedRenderPipelineId {
        let key = BatchMesh::key(&batch.geometry, &batch.material);
        if let Some(id) = self.cache.get(&key) {
            return *id;
        }

        todo!()

        // let id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        //     label: None,
        //     layout: (),
        //     push_constant_ranges: (),
        //     vertex: (),
        //     fragment: (),
        //     depth_stencil: (),
        //     multisample: (),
        //     primitive: (),
        //     zero_initialize_workgroup_memory: false,
        // });

        // self.cache.insert(key, id);

        // id
    }
}

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
