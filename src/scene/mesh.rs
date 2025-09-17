use draft_render::{GeometryResource, MaterialResource};
use fyrox_core::variable::InheritableVariable;

use crate::{
    renderer::MeshInstanceData,
    scene::{DrawContext, DynSceneNode, DynSceneObject, Node, NodeMut, NodeRef},
};

pub struct Surface {
    pub geometry: InheritableVariable<GeometryResource>,
    pub material: InheritableVariable<MaterialResource>,
}

#[derive(Default)]
pub struct Mesh {
    node: Node,
    pub surfaces: InheritableVariable<Vec<Surface>>,
}

impl DynSceneObject for Mesh {}

impl DynSceneNode for Mesh {
    fn get_mut<'a>(&'a mut self) -> NodeMut<'a> {
        self.node.get_mut()
    }

    fn get_ref(&self) -> NodeRef {
        self.node.get_ref()
    }

    fn draw(&self, context: &mut DrawContext) {
        for surface in self.surfaces.iter() {
            context.render_data_bundle_storage.push_mesh(
                surface.geometry.clone_inner(),
                surface.material.clone_inner(),
                0,
                MeshInstanceData::from_node(&self.node),
            );
        }
    }
}
