use fyrox_core::{
    ImmutableString,
    algebra::{Matrix4, Vector3},
};

use crate::{
    renderer::CameraUniform,
    scene::{Camera, DynSceneNode, Projection},
};

#[derive(Default)]
pub struct Observer {
    pub projection: Projection,
    pub position: ObserverPosition,
    pub pipeline_name: ImmutableString,
}

#[derive(Clone, Default)]
pub struct ObserverPosition {
    pub translation: Vector3<f32>,
    pub z_near: f32,
    pub z_far: f32,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
    pub view_projection_matrix: Matrix4<f32>,
}

impl ObserverPosition {
    pub fn get_uniform(&self) -> CameraUniform {
        CameraUniform {
            view_projection_matrix: self.view_projection_matrix,
        }
    }

    pub fn from_camera(camera: &Camera) -> Self {
        Self {
            translation: camera.get_ref().global_position(),
            z_near: camera.projection().z_near(),
            z_far: camera.projection().z_far(),
            view_matrix: camera.view_matrix(),
            projection_matrix: camera.projection_matrix(),
            view_projection_matrix: camera.view_projection_matrix(),
        }
    }
}
