use std::{num::NonZero, sync::Arc};

use draft_graphics::{
    CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureUsages,
    gfx_base::{GpuSurface, GpuSurfaceTexture, RenderAdapter, RenderDevice, RenderInstance},
    wgpu::SurfaceError,
};
use fyrox_core::{
    parking_lot::{Mutex, MutexGuard},
    pool::{Handle, Pool},
};
use thiserror::Error;

use crate::{ISystemWindow, RawHandleWrapper, WindowWrapper};

const DEFAULT_DESIRED_MAXIMUM_FRAME_LATENCY: u32 = 2;

#[derive(Debug, Error)]
pub enum Error {
    #[error("System window is already initialized.")]
    SystemWindowAlreadyInitialized,
}

#[derive(Default, Clone)]
pub struct Window {
    pub physical_width: u32,
    pub physical_height: u32,
    pub present_mode: PresentMode,
    pub desired_maximum_frame_latency: Option<NonZero<u32>>,
    pub alpha_mode: CompositeAlphaMode,
}

pub struct InitializedSystemWindow {
    pub window: Window,
    pub wrapper: WindowWrapper,
    pub surface: GpuSurface,
    pub configuration: SurfaceConfiguration,
}

impl InitializedSystemWindow {
    pub fn get_current_texture(&self) -> Result<GpuSurfaceTexture, SurfaceError> {
        self.surface.get_current_texture()
    }
}

pub enum SystemWindow {
    Initialized(InitializedSystemWindow),
    Uninitialized(Window),
}

impl SystemWindow {
    pub fn get_current_texture(&self) -> Option<Result<GpuSurfaceTexture, SurfaceError>> {
        if let SystemWindow::Initialized(initialized_system_window) = self {
            Some(initialized_system_window.get_current_texture())
        } else {
            None
        }
    }

    pub fn get_window(&self) -> Option<&Window> {
        if let SystemWindow::Initialized(initialized_system_window) = self {
            Some(&initialized_system_window.window)
        } else {
            None
        }
    }

    pub fn get_window_wrapper(&self) -> Option<&WindowWrapper> {
        if let SystemWindow::Initialized(initialized_system_window) = self {
            Some(&initialized_system_window.wrapper)
        } else {
            None
        }
    }

    pub fn get_system_window<M: ISystemWindow>(&self) -> Option<&M> {
        if let SystemWindow::Initialized(initialized_system_window) = self {
            Some(initialized_system_window.wrapper.get_system_window())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct SystemWindowManagerState {
    primary: Option<Handle<SystemWindow>>,
    pool: Pool<SystemWindow>,
}

impl SystemWindowManagerState {
    fn spawn_primary(&mut self, window: Window) -> Handle<SystemWindow> {
        let handle = self.spawn(window);
        self.primary = Some(handle);

        handle
    }

    fn iter(&self) -> impl Iterator<Item = &SystemWindow> {
        self.pool.iter()
    }

    pub fn initialize_system_window(
        &mut self,
        render_instance: &RenderInstance,
        render_device: &RenderDevice,
        render_adapter: &RenderAdapter,
        handle: Handle<SystemWindow>,
        window_wrapper: WindowWrapper,
    ) -> Result<(), Error> {
        let window = match self.pool.borrow(handle) {
            SystemWindow::Uninitialized(window) => window.clone(),
            SystemWindow::Initialized(_) => {
                return Err(Error::SystemWindowAlreadyInitialized);
            }
        };

        let wrapper = RawHandleWrapper::new(&window_wrapper).expect("Invalid window.");

        // SAFETY: Plugins should be set up on the main thread.
        let handle_wrapper = unsafe { wrapper.get_handle() };

        let surface = render_instance.create_surface(handle_wrapper);
        let caps = surface.get_capabilities(&render_adapter);

        let formats = caps.formats;
        let format = *formats.first().expect("No supported formats for surface");

        let configuration = SurfaceConfiguration {
            format,
            width: window.physical_width,
            height: window.physical_height,
            usage: TextureUsages::RENDER_ATTACHMENT,
            present_mode: window.present_mode,
            desired_maximum_frame_latency: window
                .desired_maximum_frame_latency
                .map(NonZero::<u32>::get)
                .unwrap_or(DEFAULT_DESIRED_MAXIMUM_FRAME_LATENCY),
            alpha_mode: window.alpha_mode,
            view_formats: if !format.is_srgb() {
                vec![format.add_srgb_suffix()]
            } else {
                vec![]
            },
        };

        render_device.configure_surface(&surface, &configuration);

        *self.pool.borrow_mut(handle) = SystemWindow::Initialized(InitializedSystemWindow {
            window,
            wrapper: window_wrapper,
            surface,
            configuration,
        });

        Ok(())
    }

    fn spawn(&mut self, window: Window) -> Handle<SystemWindow> {
        self.pool.spawn(SystemWindow::Uninitialized(window))
    }

    pub fn remove(&mut self, handle: Handle<SystemWindow>) -> SystemWindow {
        self.pool.free(handle)
    }
}

pub struct SystemWindowManagerRef<'a> {
    guard: MutexGuard<'a, SystemWindowManagerState>,
}

impl SystemWindowManagerRef<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &SystemWindow> {
        self.guard.iter()
    }
}

#[derive(Default, Clone)]
pub struct SystemWindowManager(Arc<Mutex<SystemWindowManagerState>>);

impl SystemWindowManager {
    pub fn get_ref<'a>(&'a self) -> SystemWindowManagerRef<'a> {
        SystemWindowManagerRef {
            guard: self.0.lock(),
        }
    }

    pub fn spawn_primary(&mut self, window: Window) -> Handle<SystemWindow> {
        let mut guard = self.0.lock();
        guard.spawn_primary(window)
    }

    pub fn spawn(&mut self, window: Window) -> Handle<SystemWindow> {
        let mut guard = self.0.lock();

        guard.spawn(window)
    }

    pub fn initialize_system_window(
        &mut self,
        render_instance: &RenderInstance,
        render_device: &RenderDevice,
        render_adapter: &RenderAdapter,
        handle: Handle<SystemWindow>,
        window_wrapper: WindowWrapper,
    ) -> Result<(), Error> {
        let mut guard = self.0.lock();
        guard.initialize_system_window(
            render_instance,
            render_device,
            render_adapter,
            handle,
            window_wrapper,
        )?;

        Ok(())
    }
}
