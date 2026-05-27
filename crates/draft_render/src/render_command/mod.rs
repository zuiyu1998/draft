use std::collections::HashMap;

use draft_mesh::Mesh;

use crate::{
    render_world::{RenderWorld, ResourceId},
    renderer::MeshMaterialRenderer,
};

pub struct RenderCommandContext<'a> {
    pub render_world: &'a mut RenderWorld,
    pub mesh_material_renderer: &'a mut dyn MeshMaterialRenderer,
}

pub trait RenderCommand: 'static {
    fn execute(&self, context: &mut RenderCommandContext);
}

pub struct MeshMaterialRenderCommand {
    pub mesh_id: ResourceId<Mesh>,
}

impl RenderCommand for MeshMaterialRenderCommand {
    fn execute(&self, _context: &mut RenderCommandContext) {}
}

pub struct RenderCommands {
    commands: Vec<Box<dyn RenderCommand>>,
}

impl RenderCommands {
    pub fn add_command(&mut self, command: impl RenderCommand) {
        self.commands.push(Box::new(command));
    }

    pub fn execute(&self, context: &mut RenderCommandContext) {
        for command in self.commands.iter() {
            command.execute(context);
        }
    }
}

#[derive(Default)]
pub struct RenderCommandsContainer {
    data: HashMap<String, RenderCommands>,
}

impl RenderCommandsContainer {
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, key: &str) -> &mut RenderCommands {
        self.data
            .entry(key.to_string())
            .or_insert_with(|| RenderCommands {
                commands: Vec::new(),
            })
    }

    pub fn new() -> Self {
        let container = Self::empty();

        container
    }
}
