mod draw_state;

use draft_mesh::Mesh;

use crate::render_world::{CachePipelineId, RenderWorld, ResourceId};

pub use draw_state::*;

pub struct RenderPhase {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
}

impl RenderPhase {
    pub fn new(mesh_id: ResourceId<Mesh>, pipeline_id: CachePipelineId) -> Self {
        Self {
            mesh_id,
            pipeline_id,
        }
    }

    pub fn render(&self, builder: &mut TrackedRenderPassBuilder, _render_world: &RenderWorld) {
        builder.set_render_pipeline(self.pipeline_id);

        builder.reset();
    }
}
