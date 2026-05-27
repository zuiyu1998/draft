pub mod error;
pub mod frame_graph;
pub mod render_command;
pub mod render_pipeline;
pub mod render_world;
pub mod renderer;

use std::mem::take;

use draft_graphics::RenderServer;
use draft_window::SystemWindowManager;

pub const CORE_2D: &str = "core_2d";

use crate::{
    render_command::{RenderCommandContext, RenderCommandsContainer},
    render_pipeline::{RenderPipeline, RenderPipelineContainer, RenderPipelineRunContext},
    render_world::RenderWorld,
    renderer::{MeshMaterialRenderer2d, MeshMaterialRendererContainer},
};

pub use error::FrameworkError;

pub trait IWorld: 'static {
    fn render(&self, context: &mut RenderContext);
}

pub struct RenderContext<'a> {
    pub render_world: &'a mut RenderWorld,
    pub render_commands_container: &'a mut RenderCommandsContainer,
}

pub struct WorldRenderer {
    pub render_server: RenderServer,
    pub system_window_manager: SystemWindowManager,
    pub render_pipeline_container: RenderPipelineContainer,
    pub render_world: RenderWorld,
    pub render_commands_container: RenderCommandsContainer,
    pub mesh_material_renderer_container: MeshMaterialRendererContainer,
}

impl WorldRenderer {
    pub fn new(render_server: RenderServer, system_window_manager: SystemWindowManager) -> Self {
        Self {
            render_world: RenderWorld::new(&render_server.device),
            render_server,
            system_window_manager,
            render_pipeline_container: RenderPipelineContainer::default(),
            render_commands_container: RenderCommandsContainer::new(),
            mesh_material_renderer_container: MeshMaterialRendererContainer::default(),
        }
    }

    pub fn initialize(&mut self) {
        self.render_pipeline_container
            .insert(CORE_2D, RenderPipeline::default());

        self.mesh_material_renderer_container
            .insert(CORE_2D, MeshMaterialRenderer2d {});
    }

    fn flush(&mut self) {
        let mut render_commands_container = take(&mut self.render_commands_container);

        for (key, mesh_material_renderer) in self.mesh_material_renderer_container.iter_mut() {
            let mut context = RenderCommandContext {
                render_world: &mut self.render_world,
                mesh_material_renderer: mesh_material_renderer.as_mut(),
            };

            let render_commands = render_commands_container.get_or_insert(&key);

            render_commands.execute(&mut context);
        }
    }

    pub fn render<W: IWorld>(&mut self, world: &W) {
        self.render_world
            .prepare_windows(&self.render_server, &self.system_window_manager);

        let mut context = RenderContext {
            render_world: &mut self.render_world,
            render_commands_container: &mut self.render_commands_container,
        };

        world.render(&mut context);

        self.flush();

        let mut context = RenderPipelineRunContext {};

        if let Some(pipeline) = self.render_pipeline_container.get(CORE_2D) {
            pipeline.run(&mut context);
        }

        self.render_world
            .clear_windows(&self.render_server, &self.system_window_manager);
    }
}
