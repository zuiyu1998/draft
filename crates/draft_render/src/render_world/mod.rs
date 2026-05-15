mod pipeline_cache;
mod render_window;
mod resource_cache;
mod temporary_cache;

use std::marker::PhantomData;

use crate::FrameworkError;
use draft_graphics::RenderServer;
use draft_mesh::{Mesh, MeshResource};
use draft_shader::{Shader, ShaderResource};
use draft_window::SystemWindowManager;

pub use pipeline_cache::*;
pub use render_window::*;
pub use resource_cache::*;
pub use temporary_cache::*;

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
    mesh_cache: ResourceCache<Mesh>,
    shader_cache: ResourceCache<Shader>,
    windows: RenderWindowContainer,
}

impl RenderWorld {
    pub fn empty() -> RenderWorld {
        Self {
            mesh_cache: ResourceCache::default(),
            shader_cache: ResourceCache::default(),
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

    pub fn get_or_create_mesh_id(
        &mut self,
        mesh: &MeshResource,
    ) -> Result<ResourceId<Mesh>, FrameworkError> {
        self.mesh_cache.get_or_create_resource_id(mesh)
    }

    pub fn get_or_create_shader_id(
        &mut self,
        shader: &ShaderResource,
    ) -> Result<ResourceId<Shader>, FrameworkError> {
        self.shader_cache.get_or_create_resource_id(shader)
    }
}
