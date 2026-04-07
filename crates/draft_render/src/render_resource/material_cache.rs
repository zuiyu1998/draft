use draft_core::{RenderResourceExt, ResourceId};
use draft_material::MaterialResource;
use draft_mesh::MeshVertexBufferLayouts;

use crate::{FrameworkError, render_resource::TemporaryCache};

pub struct MaterialRenderData {
    pub material: MaterialResource,
}

#[derive(Default)]
pub struct MaterialCache {
    cache: TemporaryCache<MaterialRenderData>,
    layouts: MeshVertexBufferLayouts,
}

fn create_material_render_data(
    _layout: &MeshVertexBufferLayouts,
    material: &MaterialResource,
) -> Result<MaterialRenderData, FrameworkError> {
    let _data = material.data_ref();

    Ok(MaterialRenderData {
        material: material.clone(),
    })
}

impl MaterialCache {
    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get_or_create_resource_id(
        &mut self,
        material_resource: &MaterialResource,
    ) -> Option<ResourceId> {
        if let Some(resource_id) = material_resource.get_resource_id() {
            if resource_id.is_none() {
                if let Some(cache_index) = material_resource.get_resource_cache_index() {
                    self.cache
                        .get_entry_mut_or_insert_with(&cache_index, Default::default(), || {
                            create_material_render_data(&self.layouts, material_resource)
                        })
                        .ok()?;

                    Some(cache_index.into())
                } else {
                    None
                }
            } else {
                Some(resource_id)
            }
        } else {
            None
        }
    }
}
