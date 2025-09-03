use draft_render::{DynamicUniformBuffer, RenderWorld, frame_graph::TransientBuffer};
use encase::ShaderType;
use fyrox_core::{
    ImmutableString,
    algebra::{Matrix4, Vector3},
};

use crate::scene::{Camera, DynNode, Projection};

#[derive(ShaderType)]
pub struct CameraUniform {
    pub view_projection_matrix: Matrix4<f32>,
}

impl CameraUniform {
    pub fn new(position: &ObserverPosition) -> Self {
        Self {
            view_projection_matrix: position.view_projection_matrix,
        }
    }
}

#[derive(Default)]
pub struct ObserversCollection {
    pub cameras: Vec<Observer>,
}

pub struct ObserversData {
    pub camera_offsets: Vec<u32>,
    pub camera_buffer: TransientBuffer,
}

impl ObserversData {
    pub fn new(collection: &ObserversCollection, render_world: &RenderWorld) -> Self {
        let mut buffer = DynamicUniformBuffer::<CameraUniform>::default();
        let mut camera_offsets = vec![];
        {
            let mut writer = buffer
                .get_writer(
                    collection.cameras.len(),
                    &render_world.server.device,
                    &render_world.server.queue,
                )
                .unwrap();

            for camera in collection.cameras.iter() {
                let uniform = CameraUniform::new(&camera.position);
                camera_offsets.push(writer.write(&uniform));
            }
        }

        let buffer = buffer.into_inner().unwrap();

        ObserversData {
            camera_offsets,
            camera_buffer: buffer,
        }
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
