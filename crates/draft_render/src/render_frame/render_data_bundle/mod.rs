mod mesh;

pub use mesh::*;

#[derive(Default)]
pub struct RenderDataBundle {
    pub mesh: BatchMeshContainer,
}

impl RenderDataBundle {
    pub fn empty() -> Self {
        RenderDataBundle::default()
    }
}
