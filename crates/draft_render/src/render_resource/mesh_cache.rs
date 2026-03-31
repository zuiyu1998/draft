use draft_core::{RenderResourceExt, ResourceId};
use draft_mesh::MeshResource;

use crate::{FrameworkError, render_resource::TemporaryCache};

pub struct MeshRenderData {
    pub mesh: MeshResource,
}

#[derive(Default)]
pub struct MeshCache {
    cache: TemporaryCache<MeshRenderData>,
}

fn create_mesh_render_data(mesh: &MeshResource) -> Result<MeshRenderData, FrameworkError> {
    Ok(MeshRenderData { mesh: mesh.clone() })
}

impl MeshCache {
    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get_or_create_resource_id(
        &mut self,
        mesh_resource: &MeshResource,
    ) -> Option<ResourceId> {
        if let Some(resource_id) = mesh_resource.get_resource_id() {
            if resource_id.is_none() {
                if let Some(cache_index) = mesh_resource.get_resource_cache_index() {
                    self.cache
                        .get_entry_mut_or_insert_with(&cache_index, Default::default(), || {
                            create_mesh_render_data(mesh_resource)
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
