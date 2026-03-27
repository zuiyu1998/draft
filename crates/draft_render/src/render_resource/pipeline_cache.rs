use draft_core::RenderResourceExt;
use draft_material::PipelineResource;

use crate::{FrameworkError, render_resource::TemporaryCache};

pub struct PipelineRenderData {
    pub pipeline: PipelineResource,
}

#[derive(Default)]
pub struct PipelineCache {
    cache: TemporaryCache<PipelineRenderData>,
}

fn create_pipeline_render_data(
    pipeline: &PipelineResource,
) -> Result<PipelineRenderData, FrameworkError> {
    Ok(PipelineRenderData {
        pipeline: pipeline.clone(),
    })
}

impl PipelineCache {
    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get(&mut self, pipeline_resource: &PipelineResource) -> Option<&PipelineRenderData> {
        if let Some(cache_index) = pipeline_resource.get_resource_cache_index() {
            match self
                .cache
                .get_mut_or_insert_with(&cache_index, Default::default(), || {
                    create_pipeline_render_data(pipeline_resource)
                }) {
                Ok(entry) => {
                    return Some(entry);
                }
                Err(_e) => None,
            }
        } else {
            None
        }
    }
}
