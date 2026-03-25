pub mod render_resource;
pub mod render_server;

pub use wgpu;

use draft_window::SystemWindowManager;
use thiserror::Error;

use crate::{
    render_resource::{WindowSurface, WindowSurfaces},
    render_server::RenderServer,
};

#[derive(Debug, Error)]
pub enum FrameworkError {}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub window_surfaces: WindowSurfaces,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_server,
            system_window_manager,
            window_surfaces: Default::default(),
        }
    }

    pub fn prepare_window_surfaces(&mut self) {
        let windows = self.system_window_manager.state().windows().clone();

        for window_handle in windows.iter() {
            let window = self
                .system_window_manager
                .state()
                .get_window(&window_handle)
                .clone();

            self.window_surfaces
                .data
                .entry(window_handle.clone())
                .and_modify(|window_surface| {
                    window_surface.configure_surface(&self.render_server.device, &window)
                })
                .or_insert_with(|| {
                    WindowSurface::new(
                        &self.render_server.instance,
                        &self.render_server.adapter,
                        &window,
                    )
                });
        }
    }

    pub fn render(&mut self) {
        self.prepare_window_surfaces();
    }
}
