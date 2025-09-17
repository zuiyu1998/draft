use std::num::NonZero;

use encase::ShaderType;
use fyrox_core::algebra::Matrix4;

use draft_render::{frame_graph::TransientBuffer, render_resource::RenderBuffer};

#[derive(ShaderType)]
pub struct CameraUniform {
    pub view_projection_matrix: Matrix4<f32>,
}

pub struct CameraUniforms {
    pub camera_offsets: Vec<u32>,
    pub camera_size: NonZero<u64>,
    camera_buffer: TransientBuffer,
}

impl CameraUniforms {
    const CAMERA_BUFFER_KEY: &'static str = "camera";

    pub fn new(
        camera_offsets: Vec<u32>,
        camera_size: NonZero<u64>,
        camera_buffer: TransientBuffer,
    ) -> Self {
        Self {
            camera_offsets,
            camera_buffer,
            camera_size,
        }
    }

    pub fn get_camera_buffer(&self) -> RenderBuffer {
        RenderBuffer {
            key: Self::CAMERA_BUFFER_KEY.into(),
            value: self.camera_buffer.resource.clone(),
            desc: self.camera_buffer.desc.clone(),
        }
    }
}
