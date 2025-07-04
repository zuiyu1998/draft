use crate::{
    PassData, PipelineLayoutCache, ShaderCache,
    gfx_base::{CachedPipelineId, GetPipelineCache, PipelineCache, RenderDevice},
};

use fyrox_core::log::Log;

use crate::{PassResource, TemporaryCache};

#[derive(Default)]
pub struct PassCache {
    pub shader_cache: ShaderCache,
    pub pipeline_layout_cache: PipelineLayoutCache,
    pub cache: TemporaryCache<PassData>,
}

impl PassCache {
    pub fn get_or_insert(
        &mut self,
        device: &RenderDevice,
        pass: &PassResource,
    ) -> Option<&PassData> {
        let mut pass_state = pass.state();

        if let Some(pass_state) = pass_state.data() {
            match self.cache.get_mut_or_insert_with(
                &pass_state.cache_index,
                Default::default(),
                || {
                    PassData::prepare(
                        &pass_state.definition,
                        device,
                        &mut self.shader_cache,
                        &mut self.pipeline_layout_cache,
                    )
                },
            ) {
                Ok(data) => {
                    data.set_cached_pipeline_id(CachedPipelineId::new(
                        pass_state.cache_index.get(),
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

impl GetPipelineCache for PassCache {
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
