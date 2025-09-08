mod phase;

use std::num::NonZero;

pub use phase::*;

use crate::{frame_graph::TransientBuffer, render_resource::RenderBuffer};

pub struct FrameContext {
    pub camera_uniforms: Option<CameraUniforms>,
    pub render_phases_containers: RenderPhasesContainers,
}

impl FrameContext {
    pub fn new(
        camera_uniforms: Option<CameraUniforms>,
        render_phases_containers: RenderPhasesContainers,
    ) -> Self {
        FrameContext {
            camera_uniforms,
            render_phases_containers,
        }
    }
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
