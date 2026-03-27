use std::sync::Arc;

use fyrox_resource::{
    Resource, ResourceData,
    core::{TypeUuidProvider, sparse::AtomicIndex},
    manager::ResourceManager,
};

pub mod pool;

pub mod collections {
    pub use fxhash::*;
}

pub trait ImportResourcePlugin: Send + Sync + 'static {
    fn import(&self, resource_manager: &ResourceManager);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceId(usize);

impl From<Arc<AtomicIndex>> for ResourceId {
    fn from(value: Arc<AtomicIndex>) -> Self {
        ResourceId(value.get())
    }
}

pub trait RenderResource: ResourceData + Sized + Default + TypeUuidProvider {
    fn get_cache_index(&self) -> &Arc<AtomicIndex>;

    fn get_resource_id(&self) -> ResourceId {
        self.get_cache_index().clone().into()
    }
}

pub trait RenderResourceExt: Sized {
    fn get_resource_cache_index(&self) -> Option<Arc<AtomicIndex>>;
}

impl<T: RenderResource> RenderResourceExt for Resource<T> {
    fn get_resource_cache_index(&self) -> Option<Arc<AtomicIndex>> {
        let guard = self.state();
        guard
            .data_ref()
            .map(|resource| resource.get_cache_index().clone())
    }
}
