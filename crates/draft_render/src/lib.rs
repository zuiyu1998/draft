pub mod error;
pub mod frame_graph;
pub mod render_phase;
pub mod render_pipeline;
pub mod render_world;
pub mod renderer_2d;

use draft_graphics::{Color, RenderServer};
use draft_mesh::Mesh;
use draft_window::SystemWindowManager;

use crate::{
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
    render_pipeline::{
        RenderPipelineContainer, RenderPipelineRunContext, initialize_2d_render_pipeline,
    },
    render_world::{CachePipelineId, RenderWorld, ResourceId},
    renderer_2d::{CORE_2D, Renderer2d},
};

pub use error::FrameworkError;

pub trait IWorld: 'static {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    render_world: &'a mut RenderWorld,
    renderer_2d: &'a mut Renderer2d,
}

impl<'a> RenderContext<'a> {
    pub fn render_world(&mut self) -> &mut RenderWorld {
        self.render_world
    }

    pub fn create_2d_render_pipeline(
        &mut self,
        mesh_id: ResourceId<Mesh>,
    ) -> Result<CachePipelineId, FrameworkError> {
        self.renderer_2d
            .create_render_pipeline(self.render_world, mesh_id)
    }

    pub fn add_render_phase(&mut self, mesh_id: ResourceId<Mesh>, pipeline_id: CachePipelineId) {
        self.renderer_2d.add_render_phase(mesh_id, pipeline_id);
    }
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
    pub renderer_2d: Renderer2d,
    pub transient_resource_cache: TransientResourceCache,
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
            renderer_2d: Default::default(),
            transient_resource_cache: TransientResourceCache::default(),
        }
    }

    pub fn initialize(&mut self) {
        self.render_pipeline_container
            .insert(CORE_2D, initialize_2d_render_pipeline());
    }

    pub fn prepare<W: IWorld>(&mut self, world: &W) {
        self.renderer_2d.unset();

        self.render_world
            .prepare_windows(&self.render_server, &self.system_window_manager);

        let mut context = RenderContext {
            render_world: &mut self.render_world,
            renderer_2d: &mut self.renderer_2d,
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
        let pipeline_container = self.render_world.get_pipeline_container();

        let mut context = RenderPipelineRunContext {
            phases: &mut self.renderer_2d.phases,
            world: &mut self.render_world,
            options: &self.options,
        };

        if let Some(pipeline) = self.render_pipeline_container.get(CORE_2D) {
            let mut frame_graph = FrameGraph::default();

            pipeline.run(&mut frame_graph, &mut context);

            frame_graph.compile();

            let mut context = FrameGraphContext::new(
                &pipeline_container,
                &self.render_server.device,
                &mut self.transient_resource_cache,
            );

            frame_graph.execute(&mut context);

            let command_buffers = context.finish();

            self.render_server.queue.submit(command_buffers);
        }
    }

    pub fn render<W: IWorld>(&mut self, world: &W) {
        self.prepare(world);

        self.render_frame();
        self.render_unset_windows();

        self.clear();
    }
}
