mod phase;

pub use phase::*;

use crate::frame_graph::TransientBuffer;

#[derive(Default)]
pub struct FrameContext {
    pub render_phases_container: RenderPhasesContainer,
    pub camera_uniforms: Option<CameraUniforms>,
}

pub struct CameraUniforms {
    pub camera_offsets: Vec<u64>,
    pub camera_buffer: TransientBuffer,
}
