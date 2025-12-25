use fyrox_core::algebra::Vector2;
use std::f32::consts::FRAC_PI_2;

use crate::{Indices, PrimitiveTopology};

use super::Mesh;

pub trait Meshable {
    type Output: MeshBuilder;

    fn mesh(&self) -> Self::Output;
}

pub trait MeshBuilder {
    fn build(&self) -> Mesh;
}

impl<T: MeshBuilder> From<T> for Mesh {
    fn from(builder: T) -> Self {
        builder.build()
    }
}

pub struct EllipseMeshBuilder {
    pub ellipse: Ellipse,
    resolution: u32,
}

impl EllipseMeshBuilder {
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }
}

impl MeshBuilder for EllipseMeshBuilder {
    fn build(&self) -> Mesh {
        let resolution = self.resolution as usize;
        let mut indices = Vec::with_capacity((resolution - 2) * 3);
        let mut positions = Vec::with_capacity(resolution);
        let normals = vec![[0.0, 0.0, 1.0]; resolution];
        let mut uvs = Vec::with_capacity(resolution);

        // Add pi/2 so that there is a vertex at the top (sin is 1.0 and cos is 0.0)
        let start_angle = FRAC_PI_2;
        let step = core::f32::consts::TAU / self.resolution as f32;

        for i in 0..self.resolution {
            // Compute vertex position at angle theta
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = f32::sin_cos(theta);
            let x = cos * self.ellipse.half_size.x;
            let y = sin * self.ellipse.half_size.y;

            positions.push([x, y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }

        for i in 1..(self.resolution - 1) {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::attribute_position(), positions)
            .with_inserted_attribute(Mesh::attribute_normal(), normals)
            .with_inserted_attribute(Mesh::attribute_uv_0(), uvs)
            .with_inserted_indices(Indices::U32(indices))
    }
}

impl Default for EllipseMeshBuilder {
    fn default() -> Self {
        Self {
            ellipse: Ellipse::default(),
            resolution: 32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ellipse {
    pub half_size: Vector2<f32>,
}

impl Default for Ellipse {
    /// Returns the default [`Ellipse`] with a half-width of `1.0` and a half-height of `0.5`.
    fn default() -> Self {
        Self {
            half_size: Vector2::new(1.0, 0.5),
        }
    }
}

impl Meshable for Ellipse {
    type Output = EllipseMeshBuilder;

    fn mesh(&self) -> Self::Output {
        EllipseMeshBuilder {
            ellipse: *self,
            ..Default::default()
        }
    }
}

impl Ellipse {
    pub const fn new(half_width: f32, half_height: f32) -> Self {
        Self {
            half_size: Vector2::new(half_width, half_height),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

pub struct CircleMeshBuilder {
    pub circle: Circle,
    pub resolution: u32,
}

impl Default for CircleMeshBuilder {
    fn default() -> Self {
        Self {
            circle: Circle::default(),
            resolution: 32,
        }
    }
}

impl MeshBuilder for CircleMeshBuilder {
    fn build(&self) -> Mesh {
        Ellipse::new(self.circle.radius, self.circle.radius)
            .mesh()
            .resolution(self.resolution)
            .build()
    }
}

impl Meshable for Circle {
    type Output = CircleMeshBuilder;

    fn mesh(&self) -> Self::Output {
        CircleMeshBuilder {
            circle: *self,
            ..Default::default()
        }
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Self {
        circle.mesh().build()
    }
}
