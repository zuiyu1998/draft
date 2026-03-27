use draft_core::RenderResourceExt;
use draft_material::MaterialResource;

use crate::{FrameworkError, render_resource::TemporaryCache};

pub struct MaterialRenderData {
    pub material: MaterialResource,
}

#[derive(Default)]
pub struct MaterialCache {
    cache: TemporaryCache<MaterialRenderData>,
}

fn create_material_render_data(
    material: &MaterialResource,
) -> Result<MaterialRenderData, FrameworkError> {
    Ok(MaterialRenderData {
        material: material.clone(),
    })
}

impl MaterialCache {
    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get(&mut self, material_resource: &MaterialResource) -> Option<&MaterialRenderData> {
        if let Some(cache_index) = material_resource.get_resource_cache_index() {
            match self
                .cache
                .get_mut_or_insert_with(&cache_index, Default::default(), || {
                    create_material_render_data(material_resource)
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
