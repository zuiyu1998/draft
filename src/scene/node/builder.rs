use fyrox_core::algebra::Matrix4;
use std::cell::Cell;

use crate::scene::SceneObject;

use super::Node;

pub struct NodeBuilder {}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder {}
    }

    pub fn build(self) -> Node {
        Node {
            global_transform: Cell::new(Matrix4::identity()),
        }
    }

    pub fn build_scene_object(self) -> SceneObject {
        SceneObject::new(self.build())
    }
}
