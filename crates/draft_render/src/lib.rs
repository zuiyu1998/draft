pub mod error;
pub mod frame_graph;
pub mod render_pipeline;
pub mod render_world;
pub mod renderer_2d;

use draft_graphics::{Color, RenderServer};
use draft_window::SystemWindowManager;

pub const CORE_2D: &str = "core_2d";

use crate::{
    render_pipeline::{RenderPipeline, RenderPipelineContainer, RenderPipelineRunContext},
    render_world::RenderWorld,
};

pub use error::FrameworkError;

pub trait IWorld: 'static {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    pub render_world: &'a mut RenderWorld,
}

#[derive(Clone)]
pub struct RenderOptions {
    pub clear_color: Color,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            clear_color: Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }
    }
}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub render_pipeline_container: RenderPipelineContainer,
    pub render_world: RenderWorld,
    pub options: RenderOptions,
}

impl WorldRenderer {
    pub fn new(
        render_server: RenderServer,
        system_window_manager: SystemWindowManager,
        options: RenderOptions,
    ) -> Self {
        Self {
            render_world: RenderWorld::new(&render_server.device),
            render_server,
            system_window_manager,
            render_pipeline_container: RenderPipelineContainer::default(),
            options,
        }
    }

    pub fn initialize(&mut self) {
        self.render_pipeline_container
            .insert(CORE_2D, RenderPipeline::default());
    }

    pub fn prepare<W: IWorld>(&mut self, world: &W) {
        self.render_world
            .prepare_windows(&self.render_server, &self.system_window_manager);

        let mut context = RenderContext {
            render_world: &mut self.render_world,
        };

        world.render(&mut context);
    }

    pub fn clear(&mut self) {
        self.render_world
            .clear_windows(&self.render_server, &self.system_window_manager);
    }

    pub fn render_unset_windows(&mut self) {
        for handle in self.render_world.create_unused_render_windows() {
            let window = self.render_world.get_window(&handle);

            let output = window.swap_chain_texture();

            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                self.render_server
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.options.clear_color),

                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
            }

            self.render_server.queue.submit(Some(encoder.finish()));
        }
    }

    pub fn render_frame(&mut self) {
        let mut context = RenderPipelineRunContext {};

        if let Some(pipeline) = self.render_pipeline_container.get(CORE_2D) {
            pipeline.run(&mut context);
        }
    }

    pub fn render<W: IWorld>(&mut self, world: &W) {
        self.prepare(world);

        self.render_frame();
        self.render_unset_windows();

        self.clear();
    }
}
