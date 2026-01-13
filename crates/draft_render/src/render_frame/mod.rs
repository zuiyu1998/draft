mod mesh_material;
mod window;

use draft_graphics::gfx_base::PipelineContainer;
pub use mesh_material::*;
pub use window::*;

pub struct RenderDataBundle {
    pub mesh_materials: BatchRenderMeshMaterialContainer,
    pub pipeline_container: PipelineContainer,
}

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub render_data_bundle: RenderDataBundle,
}
