use draft_gfx_base::CachedPipelineId;
use fyrox_core::ImmutableString;

use crate::{
    IndexRenderBuffer, MaterialEffectData, PhaseName, RenderPhase, RenderWorld,
    frame_graph::RenderPassBuilder, render_resource::RenderBuffer,
};

pub struct MeshRenderPhase {
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
    pub pipeline_id: CachedPipelineId,
    pub material_effect_data: Vec<MaterialEffectData>,
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
