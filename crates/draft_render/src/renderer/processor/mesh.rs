use std::mem::take;

use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use fxhash::FxHashMap;

use crate::MeshCache;

#[derive(Default)]
pub struct MeshProcessor {
    pub vertex_buffer_layouts: MeshVertexBufferLayouts,
    data: FxHashMap<u64, MeshResource>,
}

impl MeshProcessor {
    pub fn prepare(&mut self, mesh: &MeshResource) {
        let key = mesh.key();
        self.data.insert(key, mesh.clone());
    }

    pub fn update_cache(&mut self, cache: &mut MeshCache) {
        let tmp = take(&mut self.data);

        for mesh in tmp.into_values() {
            cache.insert_mesh(&mesh);
        }
    }
}
