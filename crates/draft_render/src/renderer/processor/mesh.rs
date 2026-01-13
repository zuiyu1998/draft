use std::mem::take;

use draft_graphics::gfx_base::{RenderDevice, RenderQueue};
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use fxhash::FxHashMap;

use crate::{BufferAllocator, MeshCache};

#[derive(Default)]
pub struct MeshProcessor {
    data: FxHashMap<u64, MeshResource>,
}

impl MeshProcessor {
    pub fn process(&mut self, mesh: &MeshResource) {
        let key = mesh.key();
        self.data.insert(key, mesh.clone());
    }

    pub fn update_cache(
        &mut self,
        cache: &mut MeshCache,
        buffer_allocator: &mut BufferAllocator,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
        mesh_vertex_buffer_layouts: &mut MeshVertexBufferLayouts,
    ) {
        let tmp = take(&mut self.data);

        for mesh in tmp.into_values() {
            cache.insert_mesh(&mesh);
        }

        cache.allocate_and_free_meshes(
            buffer_allocator,
            render_device,
            render_queue,
            mesh_vertex_buffer_layouts,
        );
    }
}
