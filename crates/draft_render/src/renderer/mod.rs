use draft_image::Image;
use draft_material::{Material, Pipeline};
use draft_mesh::Mesh;
use draft_window::SystemWindowManager;
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::sync::mpsc::Receiver;

use crate::{
    render_pipeline::{CORE_2D, RenderPipelineContext, RenderPipelineManager},
    render_resource::{RenderCamera, RenderWorld, WindowSurface, WindowSurfaces},
    render_server::RenderServer,
};

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub window_surfaces: WindowSurfaces,
    pub render_world: RenderWorld,
    pub render_pipeline_manager: RenderPipelineManager,

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
            render_pipeline_manager: Default::default(),
        }
    }

    pub fn update_caches(&mut self, resource_manager: &ResourceManager, dt: f32) {
        self.update_texture_cache(resource_manager, dt);
        self.update_mesh_cache(dt);
        self.update_pipeline_cache(dt);
        self.update_material_cache(dt);
    }

    pub fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext {
            render_world: &mut self.render_world,
        }
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
                    self.render_world.get_or_create_mesh_resource_id(&mesh);
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

    pub fn update_render(&mut self) {
        let mut context = RenderPipelineContext::new(
            &self.system_window_manager,
            &self.render_world,
            &self.render_server,
        );

        context.set_render_camera(RenderCamera::primary());

        if let Some(pipeline) = self.render_pipeline_manager.get_pipeline(CORE_2D) {
            pipeline.run(&mut context);
        }
    }

    pub fn render(&mut self) {
        self.pre_render();
        self.update_render();
        self.post_render();
    }
}

pub struct RenderContext<'a> {
    pub render_world: &'a mut RenderWorld,
}

pub trait World: 'static {
    fn render(&mut self, context: &mut RenderContext);
}
