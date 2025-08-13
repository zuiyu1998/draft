use fyrox_core::{
    ImmutableString,
    algebra::{Matrix4, Vector3},
    reflect::*,
    visitor::*,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct ObserversCollection {
    pub cameras: Vec<Observer>,
}

#[derive(Reflect, Clone, Debug, PartialEq, Visit, Serialize, Deserialize)]
pub struct PerspectiveProjection {
    #[reflect(min_value = 0.0, max_value = 6.28, step = 0.1)]
    pub fov: f32,
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_near: f32,
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_far: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self {
            fov: 75.0f32.to_radians(),
            z_near: 0.025,
            z_far: 2048.0,
        }
    }
}

#[derive(Reflect, Clone, Debug, PartialEq, Visit, Serialize, Deserialize)]
pub struct OrthographicProjection {
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_near: f32,
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_far: f32,
    #[reflect(step = 0.1)]
    pub vertical_size: f32,
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        Self {
            z_near: 0.0,
            z_far: 2048.0,
            vertical_size: 5.0,
        }
    }
}

pub enum Projection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

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
