use draft_render::{DynamicUniformBuffer, RenderWorld};
use encase::ShaderType;
use fyrox_core::{
    ImmutableString,
    algebra::{Matrix4, Vector3},
};

use crate::{
    renderer::{CameraUniform, CameraUniforms},
    scene::{Camera, DynSceneNode, Projection},
};

#[derive(Default)]
pub struct ObserversCollection {
    pub cameras: Vec<Observer>,
}

impl ObserversCollection {
    pub fn prepare(&self, render_world: &RenderWorld) -> Option<CameraUniforms> {
        if self.cameras.is_empty() {
            return None;
        }

        let mut buffer = DynamicUniformBuffer::<CameraUniform>::default();
        let mut offsets = vec![];
        {
            let mut writer = buffer
                .get_writer(
                    self.cameras.len(),
                    &render_world.server.device,
                    &render_world.server.queue,
                )
                .unwrap();

            for camera in self.cameras.iter() {
                let uniform = camera.position.get_uniform();
                offsets.push(writer.write(&uniform));
            }
        }

        let buffer = buffer.into_inner().unwrap();

        let size = CameraUniform::min_size();

        Some(CameraUniforms::new(offsets, size, buffer))
    }
}

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
