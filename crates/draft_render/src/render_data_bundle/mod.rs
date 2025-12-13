mod mesh;
mod window;

pub use mesh::*;
pub use window::*;

#[derive(Default)]
pub struct RenderDataBundle {
    pub windows: RenderWindows,
    pub mesh: BatchMeshContainer,
}

impl RenderDataBundle {
    pub fn empty() -> Self {
        RenderDataBundle::default()
    }
}
