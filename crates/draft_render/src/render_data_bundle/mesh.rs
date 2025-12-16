use crate::{
    CachedRenderPipelineId, MaterialResource, PipelineCache, PipelineState,
    RenderPipelineDescriptor, error::FrameworkError,
};
use draft_geometry::GeometryResource;
use fxhash::{FxHashMap, FxHasher};
use std::{
    collections::{HashMap, hash_map::Entry},
    hash::Hasher,
    ops::Deref,
};

#[derive(Default)]
pub struct SpecializedMeshPipeline {
    cache: HashMap<u64, CachedRenderPipelineId>,
}

impl SpecializedMeshPipeline {
    pub fn get(
        &mut self,
        batch: &BatchMesh,
        pipeline_cache: &mut PipelineCache,
    ) -> Result<CachedRenderPipelineId, FrameworkError> {
        let key = BatchMesh::key(&batch.geometry, &batch.material);
        if let Some(id) = self.cache.get(&key) {
            return Ok(*id);
        }
        if !batch.material.is_ok() {
            return Err(FrameworkError::MaterialInvalid(batch.material.summary()));
        }

        if !batch.geometry.is_ok() {
            return Err(FrameworkError::GeometryInvalid(batch.material.summary()));
        }

        let pipeline_state = batch.material.data_ref().pipeline_state.clone();

        let id = self.specialize(pipeline_state, pipeline_cache)?;

        self.cache.insert(key, id);

        Ok(id)
    }

    pub fn specialize(
        &mut self,
        pipeline_state: PipelineState,
        pipeline_cache: &mut PipelineCache,
    ) -> Result<CachedRenderPipelineId, FrameworkError> {
        //todo
        let id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout: vec![],
            push_constant_ranges: vec![],
            vertex: pipeline_state.vertex,
            fragment: pipeline_state.fragment,
            depth_stencil: pipeline_state.depth_stencil,
            multisample: pipeline_state.multisample,
            primitive: pipeline_state.primitive,
            zero_initialize_workgroup_memory: false,
        });

        Ok(id)
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

impl Deref for BatchMeshContainer {
    type Target = FxHashMap<u64, BatchMesh>;

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
