use draft_material::MaterialResource;
use draft_mesh::MeshResource;
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
}

#[derive(Debug, Clone)]
pub struct Surface {
    pub mesh: InheritableVariable<MeshResource>,
    pub material: InheritableVariable<MaterialResource>,
    pub unique_material: InheritableVariable<bool>,
}
