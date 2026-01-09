mod mesh_material;
mod window;

pub use mesh_material::*;
pub use window::*;

pub struct RenderDataBundle {
    pub mesh_materials: BatchMeshMaterialContainer,
}

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub mesh_materials: BatchRenderMeshMaterialContainer,
}
