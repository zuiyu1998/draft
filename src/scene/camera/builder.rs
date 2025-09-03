use fyrox_core::{ImmutableString, algebra::Matrix4};

use crate::scene::{Camera, NodeBuilder, Object, Projection};

pub const CAMERA_2D: &str = "2d";

pub struct CameraBuilder {
    node_builder: NodeBuilder,
    projection: Projection,
    pipeline_name: ImmutableString,
}

impl CameraBuilder {
    pub fn new(pipeline_name: ImmutableString) -> Self {
        Self {
            node_builder: NodeBuilder::default(),
            projection: Projection::default(),
            pipeline_name,
        }
    }

    pub fn new_2d() -> Self {
        Self::new(CAMERA_2D.into())
    }

    pub fn with_pipeline_name(mut self, pipeline_name: ImmutableString) -> Self {
        self.pipeline_name = pipeline_name;
        self
    }

    pub fn build(self) -> Camera {
        let node = self.node_builder.build();

        Camera {
            node,
            view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
            projection: self.projection.into(),
            pipeline_name: self.pipeline_name.clone(),
        }
    }

    pub fn build_object(self) -> Object {
        Object::new(self.build())
    }
}
