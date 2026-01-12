use std::ops::Range;

use crate::{
    BatchMeshMaterial, BatchMeshMaterialContainer, BatchMeshMaterialKey, BatchRenderMeshMaterial,
    BatchRenderMeshMaterialContainer, MaterialEffectCache, MaterialEffectInstance, PipelineCache,
    RenderMeshInfo,
};

pub struct MeshMaterialProcessor {}

pub struct MeshMaterialBufferContext {}

impl MeshMaterialBufferContext {
    pub fn new(_material_effect_instance: &MaterialEffectInstance) -> Self {
        MeshMaterialBufferContext {}
    }
}

pub struct MeshMaterialContext<'a> {
    batch_mesh_materials: &'a [BatchMeshMaterial],
    batch_range: &'a Range<u32>,
    key: &'a BatchMeshMaterialKey,
}

impl<'a> MeshMaterialContext<'a> {
    pub fn new(
        batch_mesh_materials: &'a [BatchMeshMaterial],
        batch_range: &'a Range<u32>,
        key: &'a BatchMeshMaterialKey,
        _material_effect_instance: &'a MaterialEffectInstance,
    ) -> Self {
        Self {
            batch_mesh_materials,
            batch_range,
            key,
        }
    }
}

impl MeshMaterialProcessor {
    pub fn new() -> Self {
        MeshMaterialProcessor {}
    }

    pub fn process_mesh_material(
        &self,
        context: &mut MeshMaterialContext,
    ) -> Vec<BatchRenderMeshMaterial> {
        let mut batchs = vec![];
        for batch in context.batch_mesh_materials {
            batchs.push(BatchRenderMeshMaterial {
                pipeline_id: context.key.pipeline_id.id(),
                mesh_info: RenderMeshInfo::from_mesh(&batch.mesh),
                batch_range: context.batch_range.clone(),
                bind_groups: vec![]
            });
        }
        batchs
    }

    pub fn process(
        &self,
        mesh_materials: BatchMeshMaterialContainer,
        pipeline_cache: &mut PipelineCache,
        material_effect_cache: &mut MaterialEffectCache,
    ) -> BatchRenderMeshMaterialContainer {
        let mut container = BatchRenderMeshMaterialContainer::default();

        for (key, batch_mesh_materials) in mesh_materials.iter() {
            if pipeline_cache.get_pipeline(key.pipeline_id.id()).is_none() {
                continue;
            }

            let Some(material_effect_instance) = material_effect_cache
                .get_material_effect_instance(&key.material_class.get_effect_name())
            else {
                tracing::error!(
                    "{} material effect not found.",
                    key.material_class.get_effect_name()
                );

                continue;
            };

            let batch_range = 0..(batch_mesh_materials.len() as u32);

            let mut context = MeshMaterialContext::new(
                batch_mesh_materials,
                &batch_range,
                key,
                material_effect_instance,
            );

            let batchs = self.process_mesh_material(&mut context);

            container.insert(key.clone(), batchs);
        }

        container
    }
}
