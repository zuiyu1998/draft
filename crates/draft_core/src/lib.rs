use std::sync::Arc;

use fyrox_resource::{ResourceData, core::sparse::AtomicIndex, manager::ResourceManager};

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

pub trait RenderResource: ResourceData {
    fn get_resource_id(&self) -> ResourceId;
}
