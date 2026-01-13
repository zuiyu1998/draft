use draft_material::MaterialResource;
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};

use crate::{
    MaterialEffectInstance, MeshMaterialInstanceData, MeshMaterialPipeline, PipelineCache,
};

pub struct MeshMaterialProcessor {
    mesh_material_pipeline: MeshMaterialPipeline,
}

pub struct MeshMaterialBufferContext {}

impl MeshMaterialBufferContext {
    pub fn new(_material_effect_instance: &MaterialEffectInstance) -> Self {
        MeshMaterialBufferContext {}
    }
}

impl MeshMaterialProcessor {
    pub fn new() -> Self {
        MeshMaterialProcessor {
            mesh_material_pipeline: Default::default(),
        }
    }

    pub fn process(
        &mut self,
        mesh: &MeshResource,
        material: &MaterialResource,
        _instance_data: &MeshMaterialInstanceData,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut MeshVertexBufferLayouts,
    ) {
        self.mesh_material_pipeline
            .get(mesh, material, pipeline_cache, layouts);
    }
}
