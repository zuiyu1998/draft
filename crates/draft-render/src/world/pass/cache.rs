use crate::{
    MaterialData, PipelineLayoutCache, ShaderCache,
    gfx_base::{CachedPipelineId, GetPipelineCache, Pipeline, PipelineCache, RenderDevice},
};

use fyrox_core::log::Log;

use crate::{MaterialResource, PipelineDescriptor, TemporaryCache};

#[derive(Debug, Clone)]
pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub pipeline: Pipeline,
}

#[derive(Default)]
pub struct MaterialCache {
    pub shader_cache: ShaderCache,
    pub pipeline_layout_cache: PipelineLayoutCache,
    pub cache: TemporaryCache<MaterialData>,
}

impl MaterialCache {
    pub fn get_or_insert(
        &mut self,
        device: &RenderDevice,
        material: &MaterialResource,
    ) -> Option<&MaterialData> {
        let mut material_state = material.state();

        if let Some(material_state) = material_state.data() {
            match self.cache.get_mut_or_insert_with(
                &material_state.cache_index,
                Default::default(),
                || {
                    MaterialData::prepare(
                        &material_state.definition,
                        device,
                        &mut self.shader_cache,
                        &mut self.pipeline_layout_cache,
                    )
                },
            ) {
                Ok(data) => {
                    data.set_cached_pipeline_id(CachedPipelineId::new(
                        material_state.cache_index.get(),
                    ));
                    Some(data)
                }
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}

impl GetPipelineCache for MaterialCache {
    fn get_pipeline_cache(&self) -> PipelineCache {
        let mut target = vec![];
        for index in 0..self.cache.buffer.len() {
            let pipeline = self
                .cache
                .buffer
                .get_raw(index)
                .map(|entry| entry.value.get_pipeline());

            target.push(pipeline);
        }

        PipelineCache::new(target)
    }
}
