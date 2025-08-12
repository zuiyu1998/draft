use fyrox_core::ImmutableString;

use crate::{
    IndexRenderBuffer, PhaseName, RenderPhase, RenderWorld, frame_graph::RenderPassBuilder,
    render_resource::RenderBuffer,
};

pub struct MeshRenderPhase {
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
}

impl PhaseName for MeshRenderPhase {
    fn name() -> fyrox_core::ImmutableString {
        ImmutableString::new("MeshRenderPhase")
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, _render_pass_builder: &mut RenderPassBuilder, _world: &mut RenderWorld) {
        todo!()
    }
}
