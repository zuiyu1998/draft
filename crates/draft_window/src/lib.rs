use downcast_rs::{Downcast, impl_downcast};
use draft_core::pool::Pool;
use std::sync::{Arc, Mutex};

pub struct Window {}

pub struct SystemWindowManager(Arc<Mutex<SystemWindowManagerState>>);

pub struct SystemWindowManagerState {
    pool: Pool<SystemWindow>,
}

pub struct SystemWindow {
    reference: Arc<dyn ISystemWindow>,
}

impl SystemWindow {
    pub fn downcast_ref<T: ISystemWindow>(&self) -> Option<&T> {
        self.reference.downcast_ref()
    }
}

pub trait ISystemWindow: 'static + Send + Sync + Downcast {}

impl_downcast!(ISystemWindow);
