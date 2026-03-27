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

    pub fn get(&mut self, mesh_resource: &MeshResource) -> Option<&MeshRenderData> {
        if let Some(cache_index) = {
            let mesh_data_guard = mesh_resource.state();
            mesh_data_guard
                .data_ref()
                .map(|mesh| mesh.cache_index.clone())
        } {
            match self
                .cache
                .get_mut_or_insert_with(&cache_index, Default::default(), || {
                    create_mesh_render_data(mesh_resource)
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
