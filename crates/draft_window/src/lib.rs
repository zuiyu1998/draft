use std::{any::Any, sync::Arc};

use fyrox_core::{
    parking_lot::Mutex,
    pool::{Handle, Pool},
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Window {}

pub trait ISystemWindow: 'static + Any + Send + Sync + HasWindowHandle + HasDisplayHandle {}

impl<T> ISystemWindow for T where T: 'static + Any + Send + Sync + HasWindowHandle + HasDisplayHandle
{}

pub struct SystemWindow(Arc<dyn ISystemWindow>);

impl Clone for SystemWindow {
    fn clone(&self) -> Self {
        SystemWindow(self.0.clone())
    }
}

impl SystemWindow {
    pub fn new<W: ISystemWindow>(window: W) -> Self {
        Self(Arc::new(window))
    }

    pub fn get_window(&self) -> Arc<dyn ISystemWindow> {
        self.0.clone()
    }
}

#[derive(Default)]
pub struct SystemWindowManager {
    state: Arc<Mutex<SystemWindowManagerState>>,
}

impl SystemWindowManager {
    pub fn spawn_primary_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let mut guard = self.state.lock();
        guard.spawn_primary_window(window)
    }

    pub fn spawn_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let mut guard = self.state.lock();
        guard.spawn_window(window)
    }
}

impl Clone for SystemWindowManager {
    fn clone(&self) -> Self {
        SystemWindowManager {
            state: self.state.clone(),
        }
    }
}

#[derive(Default)]
pub struct SystemWindowManagerState {
    primary: Handle<SystemWindow>,
    pool: Pool<SystemWindow>,
}

impl SystemWindowManagerState {
    pub fn spawn_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        self.pool.spawn(window)
    }

    pub fn spawn_primary_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let handle = self.spawn_window(window);

        self.primary = handle;
        handle
    }
}
