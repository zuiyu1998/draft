#[cfg(feature = "winit")]
mod winit;

use downcast_rs::{Downcast, impl_downcast};
use draft_core::{
    parking_lot::{Mutex, MutexGuard},
    pool::{Handle, Pool},
};
use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Default, Clone)]
pub struct Window {}

#[derive(Clone, Default)]
pub struct SystemWindowManager {
    state: Arc<Mutex<SystemWindowManagerState>>,
}

impl SystemWindowManager {
    pub fn state(&self) -> MutexGuard<'_, SystemWindowManagerState> {
        self.state.lock()
    }
}

pub struct SystemWindowManagerState {
    primary: Handle<SystemWindow>,
    pool: Pool<SystemWindow>,
    windows: HashSet<Handle<SystemWindow>>,
}

impl Default for SystemWindowManagerState {
    fn default() -> Self {
        SystemWindowManagerState {
            primary: Handle::NONE,
            pool: Pool::new(),
            windows: HashSet::new(),
        }
    }
}

impl SystemWindowManagerState {
    pub fn get_window(&self, handle: &Handle<SystemWindow>) -> SystemWindow {
        self.pool.get(*handle).clone()
    }

    pub fn windows(&self) -> &HashSet<Handle<SystemWindow>> {
        &self.windows
    }

    pub fn spawn_primary_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let handle = self.spawn_window(window);
        self.primary = handle;

        handle
    }

    pub fn spawn_window(&mut self, window: SystemWindow) -> Handle<SystemWindow> {
        let handle = self.pool.spawn(window);
        self.windows.insert(handle);
        handle
    }
}

#[derive(Clone)]
pub struct SystemWindow {
    reference: Arc<dyn ISystemWindow>,
}

impl SystemWindow {
    pub fn new<T: ISystemWindow>(window: T) -> Self {
        Self {
            reference: Arc::new(window),
        }
    }

    pub fn downcast_ref<T: ISystemWindow>(&self) -> Option<&T> {
        self.reference.downcast_ref()
    }

    pub fn inner_size(&self) -> PhysicalSize {
        self.reference.inner_size()
    }
}

pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

pub trait ISystemWindow:
    'static + Send + Sync + Downcast + HasWindowHandle + HasDisplayHandle
{
    fn inner_size(&self) -> PhysicalSize;
}

impl_downcast!(ISystemWindow);

#[derive(Clone)]
pub struct RawHandleWrapper {
    /// A shared reference to the window.
    /// This allows us to extend the lifetime of the window,
    /// so it doesn’t get eagerly dropped while a pipelined
    /// renderer still has frames in flight that need to draw to it.
    _window: Arc<dyn ISystemWindow>,
    /// Raw handle to a window.
    window_handle: RawWindowHandle,
    /// Raw handle to the display server.
    display_handle: RawDisplayHandle,
}

impl RawHandleWrapper {
    /// Creates a `RawHandleWrapper` from a `WindowWrapper`.
    pub fn new(window: &SystemWindow) -> Result<RawHandleWrapper, HandleError> {
        Ok(RawHandleWrapper {
            _window: window.reference.clone(),
            window_handle: window.reference.window_handle()?.as_raw(),
            display_handle: window.reference.display_handle()?.as_raw(),
        })
    }

    /// Gets the stored window handle.
    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }

    /// Sets the window handle.
    ///
    /// # Safety
    ///
    /// The passed in [`RawWindowHandle`] must be a valid window handle.
    // NOTE: The use of an explicit setter instead of a getter for a mutable reference is to limit the amount of time unsoundness can happen.
    //       If we handed out a mutable reference the user would have to maintain safety invariants throughout its lifetime. For consistency
    //       we also prefer to handout copies of the handles instead of immutable references.
    pub unsafe fn set_window_handle(&mut self, window_handle: RawWindowHandle) -> &mut Self {
        self.window_handle = window_handle;

        self
    }

    /// Gets the stored display handle
    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }

    /// Sets the display handle.
    ///
    /// # Safety
    ///
    /// The passed in [`RawDisplayHandle`] must be a valid display handle.
    pub fn set_display_handle(&mut self, display_handle: RawDisplayHandle) -> &mut Self {
        self.display_handle = display_handle;

        self
    }
}

// SAFETY: [`RawHandleWrapper`] is just a normal "raw pointer", which doesn't impl Send/Sync. However the pointer is only
// exposed via an unsafe method that forces the user to make a call for a given platform. (ex: some platforms don't
// support doing window operations off of the main thread).
// A recommendation for this pattern (and more context) is available here:
// https://github.com/rust-windowing/raw-window-handle/issues/59
unsafe impl Send for RawHandleWrapper {}
// SAFETY: This is safe for the same reasons as the Send impl above.
unsafe impl Sync for RawHandleWrapper {}
