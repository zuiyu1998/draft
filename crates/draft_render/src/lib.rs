pub mod frame_graph;
pub mod render_resource;
pub mod render_server;

pub use wgpu;

use draft_window::SystemWindowManager;
use thiserror::Error;

use crate::{
    render_resource::{RenderWorld, WindowSurface, WindowSurfaces},
    render_server::RenderServer,
};

#[derive(Debug, Error)]
pub enum FrameworkError {}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub window_surfaces: WindowSurfaces,
    pub render_world: RenderWorld,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_server,
            system_window_manager,
            window_surfaces: Default::default(),
            render_world: Default::default(),
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
                .or_insert_with(|| {
                    WindowSurface::new(
                        &self.render_server.instance,
                        &self.render_server.adapter,
                        &window,
                    )
                })
                .configure_surface(&self.render_server.device, &window);
        }
    }

    pub fn pre_render(&mut self) {
        self.prepare_window_surfaces();
        self.render_world
            .prepare_window_surface_textures(&self.window_surfaces);
    }

    pub fn post_render(&mut self) {
        self.system_window_manager.state().pre_present_notify();
        self.render_world.clear_window_surface_textures();
    }

    pub fn render(&mut self) {
        self.pre_render();

        let window_handle = self
            .system_window_manager
            .state()
            .get_primary_window_handle();

        let window_surface_texture = self.render_world
            .window_surface_textures
            .get_window_surface_texture(&window_handle)
            .unwrap();

        let texture_view =
            window_surface_texture
                .surface
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    ..Default::default()
                });

        // Renders a GREEN screen
        let mut encoder = self
            .render_server
            .device
            .wgpu_device()
            .create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        drop(renderpass);

        self.render_server.queue.0.submit([encoder.finish()]);

        self.post_render();
    }
}
