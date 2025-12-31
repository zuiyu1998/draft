mod mesh_material;
mod window;

pub use mesh_material::*;
pub use window::*;

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub mesh_materials: BatchRenderMeshMaterialContainer,
}
