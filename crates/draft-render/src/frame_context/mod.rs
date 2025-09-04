mod phase;

pub use phase::*;

use crate::{frame_graph::TransientBuffer, render_resource::RenderBuffer};

#[derive(Default)]
pub struct FrameContext {
    pub render_phases_container: RenderPhasesContainer,
    pub camera_uniforms: Option<CameraUniforms>,
}

pub struct CameraUniforms {
    camera_offsets: Vec<u32>,
    camera_buffer: TransientBuffer,
}

impl CameraUniforms {
    const CAMERA_BUFFER_KEY: &'static str = "camera";

    pub fn new(camera_offsets: Vec<u32>, camera_buffer: TransientBuffer) -> Self {
        Self {
            camera_offsets,
            camera_buffer,
        }
    }

    pub fn get_camera_buffer(&self) -> RenderBuffer {
        RenderBuffer {
            key: Self::CAMERA_BUFFER_KEY.into(),
            value: self.camera_buffer.resource.clone(),
            desc: self.camera_buffer.desc.clone(),
        }
    }

    pub fn get_camera_offset(&self, index: usize) -> Option<u32> {
        self.camera_offsets.get(index).copied()
    }
}
