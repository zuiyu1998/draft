mod mesh_cache;
mod render_window;
mod temporary_cache;
mod pipeline_cache;
mod shader_cache;

use std::marker::PhantomData;

use crate::FrameworkError;
use draft_graphics::{RenderDevice, RenderServer};
use draft_mesh::{Mesh, MeshResource};
use draft_window::SystemWindowManager;

pub use mesh_cache::*;
pub use render_window::*;
pub use temporary_cache::*;
pub use pipeline_cache::*;
pub use shader_cache::*;

pub struct ResourceId<T> {
    pub slot: usize,
    _marker: PhantomData<T>,
}

impl<T> ResourceId<T> {
    const INVAID: ResourceId<T> = ResourceId {
        slot: usize::MAX,
        _marker: PhantomData,
    };

    pub fn new(slot: usize) -> Self {
        ResourceId {
            slot,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for ResourceId<T> {
    fn default() -> Self {
        ResourceId::INVAID
    }
}

pub struct RenderWorld {
    mesh_cache: MeshCache,
    windows: RenderWindowContainer,
}

impl RenderWorld {
    pub fn empty() -> RenderWorld {
        Self {
            mesh_cache: MeshCache::default(),
            windows: RenderWindowContainer::default(),
        }
    }

    pub fn prepare_windows(
        &mut self,
        render_server: &RenderServer,
        system_window_manager: &SystemWindowManager,
    ) {
        for (handle, system_window) in system_window_manager.state().pool().pair_iter() {
            let render_window = self
                .windows
                .get_or_create(render_server, handle, system_window);

            render_window.spawn_swapchain_texture();
        }
    }

    pub fn clear_windows(
        &mut self,
        render_server: &RenderServer,
        system_window_manager: &SystemWindowManager,
    ) {
        for (handle, system_window) in system_window_manager.state().pool().pair_iter() {
            let render_window = self
                .windows
                .get_or_create(render_server, handle, system_window);

            render_window.clear_swapchain_texture();
        }
    }

    pub fn get_create_mesh(
        &mut self,
        mesh: &MeshResource,
        device: &RenderDevice,
    ) -> Result<ResourceId<Mesh>, FrameworkError> {
        self.mesh_cache.get_create_mesh(mesh, device)
    }
}
