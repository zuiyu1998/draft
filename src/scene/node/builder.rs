use fyrox_core::algebra::Matrix4;
use std::cell::Cell;

use crate::scene::AbstractNode;

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

    pub fn build_abstract_node(self) -> AbstractNode {
        AbstractNode::new(self.build())
    }
}
