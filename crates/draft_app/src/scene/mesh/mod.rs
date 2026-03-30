use draft_material::MaterialResource;
use draft_mesh::MeshResource;
use draft_render::renderer::RenderContext;
use fyrox_core::variable::InheritableVariable;

use crate::scene::{Node, SceneNodeBehavior};

#[derive(Debug, Clone)]
pub struct Mesh2d {
    pub node: Node,
    pub surfaces: InheritableVariable<Vec<Surface>>,
}

impl SceneNodeBehavior for Mesh2d {
    fn get_node_ref(&self) -> &Node {
        &self.node
    }

    fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    fn clone_boxed(&self) -> Box<dyn SceneNodeBehavior> {
        Box::new(self.clone())
    }

    fn render(&mut self, context: &mut RenderContext) {
        for surface in self.surfaces.get_value_ref().iter() {
            let mesh_id = context
                .render_world
                .get_or_create_mesh_resource_id(&surface.mesh)
                .expect("Resource id should be created");

            println!("Render mesh with id: {:?}", mesh_id);
            //todo
        }
    }
}

#[derive(Debug, Clone)]
pub struct Surface {
    pub mesh: InheritableVariable<MeshResource>,
    pub material: InheritableVariable<MaterialResource>,
    pub unique_material: InheritableVariable<bool>,
}
