mod core_2d;

use std::collections::HashMap;

pub use core_2d::*;
use draft_window::SystemWindowManager;

use crate::{render_resource::RenderWorld, render_server::RenderServer};

pub struct RenderPipelineManager {
    data: HashMap<String, RenderPipeline>,
}

impl Default for RenderPipelineManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.add_pipeline(CORE_2D, RenderPipeline::new());

        manager
    }
}

impl RenderPipelineManager {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        self.data.insert(name.to_string(), pipeline);
    }

    pub fn get_pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.data.get(name)
    }
}

pub struct RenderPipelineContext<'a> {
    system_window_manager: &'a SystemWindowManager,
    render_world: &'a RenderWorld,
    render_server: &'a RenderServer,
}

impl<'a> RenderPipelineContext<'a> {
    pub fn new(system_window_manager: &'a SystemWindowManager, render_world: &'a RenderWorld, render_server: &'a RenderServer) -> Self {
        Self {
            system_window_manager,
            render_world,
            render_server,
        }
    }
}

pub struct RenderPipeline {}

impl RenderPipeline {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, context: &mut RenderPipelineContext) {
        let window_handle = context
            .system_window_manager
            .state()
            .get_primary_window_handle();

        let window_surface_texture = context
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
        let mut encoder = context
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

        context.render_server.queue.0.submit([encoder.finish()]);
    }
}
