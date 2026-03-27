pub mod frame_graph;
pub mod render_resource;
pub mod render_server;

pub use wgpu;

use draft_image::Image;
use draft_material::{Material, Pipeline};
use draft_mesh::Mesh;
use draft_window::SystemWindowManager;
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;
use thiserror::Error;

use crate::{
    render_resource::{RenderWorld, WindowSurface, WindowSurfaces},
    render_server::RenderServer,
};

#[derive(Debug, Error)]
pub enum FrameworkError {
    #[error("Custom error is {0}")]
    Custom(String),
}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub window_surfaces: WindowSurfaces,
    pub render_world: RenderWorld,

    texture_event_receiver: Receiver<ResourceEvent>,
    mesh_event_receiver: Receiver<ResourceEvent>,
    pipeline_event_receiver: Receiver<ResourceEvent>,
    material_event_receiver: Receiver<ResourceEvent>,
}

impl WorldRenderer {
    pub fn new(
        render_server: RenderServer,
        system_window_manager: SystemWindowManager,
        resource_manager: &ResourceManager,
    ) -> Self {
        let (texture_event_sender, texture_event_receiver) = std::sync::mpsc::channel();
        resource_manager
            .state()
            .event_broadcaster
            .add(texture_event_sender);

        let (mesh_event_sender, mesh_event_receiver) = std::sync::mpsc::channel();
        resource_manager
            .state()
            .event_broadcaster
            .add(mesh_event_sender);

        let (pipeline_event_sender, pipeline_event_receiver) = std::sync::mpsc::channel();
        resource_manager
            .state()
            .event_broadcaster
            .add(pipeline_event_sender);

        let (material_event_sender, material_event_receiver) = std::sync::mpsc::channel();
        resource_manager
            .state()
            .event_broadcaster
            .add(material_event_sender);

        Self {
            render_server,
            system_window_manager,
            window_surfaces: Default::default(),
            render_world: Default::default(),
            texture_event_receiver,
            mesh_event_receiver,
            pipeline_event_receiver,
            material_event_receiver,
        }
    }

    pub fn update_caches(&mut self, resource_manager: &ResourceManager, dt: f32) {
        self.update_texture_cache(resource_manager, dt);
        self.update_mesh_cache(dt);
        self.update_pipeline_cache(dt);
        self.update_material_cache(dt);
    }

    fn update_material_cache(&mut self, dt: f32) {
        while let Ok(event) = self.material_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(material) = resource.try_cast::<Material>() {
                    self.render_world.upload_material(&material);
                }
            }
        }

        self.render_world.update_material_cache(dt);
    }

    fn update_pipeline_cache(&mut self, dt: f32) {
        while let Ok(event) = self.pipeline_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(pipeline) = resource.try_cast::<Pipeline>() {
                    self.render_world.upload_pipeline(&pipeline);
                }
            }
        }

        self.render_world.update_pipeline_cache(dt);
    }

    fn update_mesh_cache(&mut self, dt: f32) {
        while let Ok(event) = self.mesh_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(mesh) = resource.try_cast::<Mesh>() {
                    self.render_world.upload_mesh(&mesh);
                }
            }
        }

        self.render_world.update_texture_cache(dt);
    }

    fn update_texture_cache(&mut self, resource_manager: &ResourceManager, dt: f32) {
        // Maximum amount of textures uploaded to GPU per frame. This defines throughput **only** for
        // requests from resource manager. This is needed to prevent huge lag when there are tons of
        // requests, so this is some kind of work load balancer.
        const THROUGHPUT: usize = 5;

        let mut uploaded = 0;
        while let Ok(event) = self.texture_event_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(texture) = resource.try_cast::<Image>() {
                    match self.render_world.upload_texture(
                        &self.render_server.device,
                        resource_manager,
                        &texture,
                    ) {
                        Ok(_) => {
                            uploaded += 1;
                            if uploaded >= THROUGHPUT {
                                break;
                            }
                        }
                        Err(e) => {
                            println!("Renderer update texture cache faild.The error is: {e}");
                        }
                    }
                }
            }
        }

        self.render_world.update_texture_cache(dt);
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

        let window_surface_texture = self
            .render_world
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
