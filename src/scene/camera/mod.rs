mod builder;

pub use builder::*;

use crate::{
    renderer::{Observer, ObserverPosition},
    scene::{DrawContext, DynSceneObject},
};

use super::{DynSceneNode, Node, NodeMut, NodeRef};
use fyrox_core::{
    ImmutableString,
    algebra::{Matrix4, Point3, Vector2},
    reflect::*,
    variable::InheritableVariable,
    visitor::*,
};
use serde::{Deserialize, Serialize};

#[derive(Reflect, Clone, Debug, PartialEq, Visit, Serialize, Deserialize)]
pub struct PerspectiveProjection {
    #[reflect(min_value = 0.0, max_value = 6.28, step = 0.1)]
    pub fov: f32,
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_near: f32,
    #[reflect(min_value = 0.0, step = 0.1)]
    pub z_far: f32,
}

impl PerspectiveProjection {
    /// Returns perspective projection matrix.
    #[inline]
    pub fn matrix(&self, frame_size: Vector2<f32>) -> Matrix4<f32> {
        let limit = 10.0 * f32::EPSILON;

        let z_near = self.z_far.min(self.z_near);
        let mut z_far = self.z_far.max(self.z_near);

        // Prevent planes from superimposing which could cause panic.
        if z_far - z_near < limit {
            z_far += limit;
        }

        Matrix4::new_perspective(
            (frame_size.x / frame_size.y).max(limit),
            self.fov,
            z_near,
            z_far,
        )
    }
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

impl OrthographicProjection {
    /// Returns orthographic projection matrix.
    #[inline]
    pub fn matrix(&self, frame_size: Vector2<f32>) -> Matrix4<f32> {
        fn clamp_to_limit_signed(value: f32, limit: f32) -> f32 {
            if value < 0.0 && -value < limit {
                -limit
            } else if value >= 0.0 && value < limit {
                limit
            } else {
                value
            }
        }

        let limit = 10.0 * f32::EPSILON;

        let aspect = (frame_size.x / frame_size.y).max(limit);

        // Prevent collapsing projection "box" into a point, which could cause panic.
        let vertical_size = clamp_to_limit_signed(self.vertical_size, limit);
        let horizontal_size = clamp_to_limit_signed(aspect * vertical_size, limit);

        let z_near = self.z_far.min(self.z_near);
        let mut z_far = self.z_far.max(self.z_near);

        // Prevent planes from superimposing which could cause panic.
        if z_far - z_near < limit {
            z_far += limit;
        }

        let left = -horizontal_size;
        let top = vertical_size;
        let right = horizontal_size;
        let bottom = -vertical_size;
        Matrix4::new_orthographic(left, right, bottom, top, z_near, z_far)
    }
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

#[derive(Clone)]
pub enum Projection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

impl Projection {
    pub fn matrix(&self, frame_size: Vector2<f32>) -> Matrix4<f32> {
        match self {
            Projection::Perspective(v) => v.matrix(frame_size),
            Projection::Orthographic(v) => v.matrix(frame_size),
        }
    }

    pub fn z_near(&self) -> f32 {
        match self {
            Projection::Perspective(v) => v.z_near,
            Projection::Orthographic(v) => v.z_near,
        }
    }

    pub fn z_far(&self) -> f32 {
        match self {
            Projection::Perspective(v) => v.z_far,
            Projection::Orthographic(v) => v.z_far,
        }
    }
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

pub struct Camera {
    node: Node,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    projection: InheritableVariable<Projection>,
    pub pipeline_name: ImmutableString,
}

impl DynSceneObject for Camera {}

impl DynSceneNode for Camera {
    fn get_ref(&self) -> NodeRef {
        self.node.get_ref()
    }

    fn get_mut<'a>(&'a mut self) -> NodeMut<'a> {
        self.node.get_mut()
    }

    fn draw(&self, context: &mut DrawContext) {
        context.observers_collection.cameras.push(Observer {
            pipeline_name: self.pipeline_name.clone(),
            position: ObserverPosition::from_camera(self),
            projection: self.projection.clone_inner(),
        });
    }
}

impl Camera {
    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix * self.view_matrix
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.view_matrix
    }

    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    pub fn calculate_matrices(&mut self, frame_size: Vector2<f32>) {
        let node_ref = self.node.get_ref();

        let pos = node_ref.global_position();
        let look = node_ref.look_vector();
        let up = node_ref.up_vector();

        self.view_matrix = Matrix4::look_at_rh(&Point3::from(pos), &Point3::from(pos + look), &up);
        self.projection_matrix = self.projection.matrix(frame_size);
    }
}
