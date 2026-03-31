use draft_core::{ResourceId, collections::FxHashMap};

use crate::{render_resource::MeshCache, render_server::RenderDevice};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct MeshMaterialKey {
    mesh_id: ResourceId,
    material_id: ResourceId,
}

pub struct MeshMaterialRenderData {}

fn create_mesh_material_render_data(
    _device: &RenderDevice,
    mesh_id: &ResourceId,
    mesh_cache: &MeshCache,
) -> MeshMaterialRenderData {
    let _ = mesh_cache.get(mesh_id);

    MeshMaterialRenderData {}
}

pub struct MeshMaterialCache {
    data: FxHashMap<MeshMaterialKey, MeshMaterialRenderData>,
}

impl MeshMaterialCache {
    pub fn get_or_create(
        &mut self,
        device: &RenderDevice,
        mesh_id: ResourceId,
        material_id: ResourceId,
        mesh_cache: &MeshCache,
    ) -> &MeshMaterialRenderData {
        let key = MeshMaterialKey {
            mesh_id,
            material_id,
        };
        self.data
            .entry(key)
            .or_insert_with(|| create_mesh_material_render_data(device, &mesh_id, mesh_cache))
    }
}
