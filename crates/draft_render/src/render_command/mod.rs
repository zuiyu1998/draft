use std::collections::HashMap;

use draft_mesh::Mesh;

use crate::render_world::ResourceId;

pub const CORE_2D: &str = "core_2d";

pub trait RenderCommand: 'static {
    fn execute(&self);
}

pub struct MeshMaterialRenderCommand {
    pub mesh_id: ResourceId<Mesh>,
}

impl RenderCommand for MeshMaterialRenderCommand {
    fn execute(&self) {}
}

pub struct RenderCommands {
    pub commands: Vec<Box<dyn RenderCommand>>,
}

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
        let mut container = Self::empty();

        container.get_or_insert(CORE_2D);

        container
    }
}
