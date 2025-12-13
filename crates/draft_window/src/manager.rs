use fyrox_core::pool::{Handle, Pool};
use thiserror::Error;

use crate::{ISystemWindow, WindowWrapper};

#[derive(Default, Clone)]
pub struct Window {}

pub struct InitializedSystemWindow {
    pub window: Window,
    pub wrapper: WindowWrapper,
}

pub enum SystemWindow {
    Initialized(InitializedSystemWindow),
    Uninitialized(Window),
}

impl SystemWindow {
    pub fn get_system_window<M: ISystemWindow>(&self) -> Option<&M> {
        if let SystemWindow::Initialized(initialized_system_window) = self {
            Some(initialized_system_window.wrapper.get_system_window())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct SystemWindowManager {
    primary: Option<Handle<SystemWindow>>,
    pool: Pool<SystemWindow>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("System window is already initialized.")]
    SystemWindowAlreadyInitialized,
}

impl SystemWindowManager {
    pub fn spawn_primary(&mut self, window: Window) -> Handle<SystemWindow> {
        let handle = self.spawn(window);
        self.primary = Some(handle);

        handle
    }

    pub fn iter(&self) -> impl Iterator<Item = &SystemWindow> {
        self.pool.iter()
    }

    pub fn initialize_system_window(
        &mut self,
        handle: Handle<SystemWindow>,
        window_wrapper: WindowWrapper,
    ) -> Result<(), Error> {
        let window = match self.pool.borrow(handle) {
            SystemWindow::Uninitialized(window) => window.clone(),
            SystemWindow::Initialized(_) => {
                return Err(Error::SystemWindowAlreadyInitialized);
            }
        };

        *self.pool.borrow_mut(handle) = SystemWindow::Initialized(InitializedSystemWindow {
            window,
            wrapper: window_wrapper,
        });

        Ok(())
    }

    pub fn spawn(&mut self, window: Window) -> Handle<SystemWindow> {
        self.pool.spawn(SystemWindow::Uninitialized(window))
    }

    pub fn remove(&mut self, handle: Handle<SystemWindow>) -> SystemWindow {
        self.pool.free(handle)
    }
}
