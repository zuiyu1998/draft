mod mesh_cache;
mod temporary_cache;

use std::marker::PhantomData;

use crate::FrameworkError;
use draft_graphics::RenderDevice;
use draft_mesh::{Mesh, MeshResource};

pub use mesh_cache::*;
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
    mesh_cache: MeshCache,
}

impl RenderWorld {
    pub fn empty() -> RenderWorld {
        Self {
            mesh_cache: MeshCache::default(),
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
