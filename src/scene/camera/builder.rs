use fyrox_core::algebra::Matrix4;

use crate::scene::{AbstractNode, Camera, NodeBuilder, Projection};

pub struct CameraBuilder {
    node_builder: NodeBuilder,
    projection: Projection,
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let node = self.node_builder.build();

        Camera {
            node,
            view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
            projection: self.projection.into(),
        }
    }

    pub fn build_abstract_node(self) -> AbstractNode {
        AbstractNode::new(self.build())
    }
}
