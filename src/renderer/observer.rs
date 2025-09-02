use fyrox_core::algebra::{Matrix4, Vector3};

use crate::{renderer::PipelineName, scene::Projection};

#[derive(Default)]
pub struct ObserversCollection {
    pub cameras: Vec<Observer>,
}

#[derive(Default)]
pub struct Observer {
    pub projection: Projection,
    pub position: ObserverPosition,
    pub pipeline_name: PipelineName,
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
