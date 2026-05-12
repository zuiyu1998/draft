use std::{any::Any, sync::Arc};

use fyrox_core::{
    SafeLock,
    parking_lot::{Mutex, MutexGuard},
    pool::{Handle, Pool},
};
pub use raw_window_handle::{HasDisplayHandle, RawDisplayHandle, RawWindowHandle};

pub struct Window {}

pub trait ISystemWindow: 'static + Any + Send + Sync {
    fn get_physical_size(&self) -> PhysicalSize;
    fn get_raw_window_handle(&self) -> RawWindowHandle;
    fn get_raw_display_handle(&self) -> RawDisplayHandle;
}

pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

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
    pub fn state(&self) -> MutexGuard<'_, SystemWindowManagerState> {
        self.state.safe_lock()
    }

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
    pub fn pool(&self) -> &Pool<SystemWindow> {
        &self.pool
    }

    pub fn spawn_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        self.pool.spawn(window)
    }

    pub fn spawn_primary_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let handle = self.spawn_window(window);

        self.primary = handle;
        handle
    }
}
