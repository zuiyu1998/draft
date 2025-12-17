mod render_data_bundle;

pub use render_data_bundle::*;

use draft_geometry::GeometryVertexBufferLayouts;

use crate::{PipelineCache, error::FrameworkError};

pub struct Frame {
    pub render_data_bundle: RenderDataBundle,
}

impl Frame {
    pub fn prepare(
        &self,
        specialized_mesh_pipeline: &mut SpecializedMeshPipeline,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut GeometryVertexBufferLayouts,
    ) -> Result<RenderFrame, FrameworkError> {
        for batch in self.render_data_bundle.mesh.values() {
            specialized_mesh_pipeline.get(batch, pipeline_cache, layouts)?;
        }

        Ok(RenderFrame {})
    }
}

pub struct RenderFrame {}
