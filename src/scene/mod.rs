pub mod camera;
pub mod mesh;
pub mod node;
pub mod object;

pub use camera::*;
pub use mesh::*;
pub use node::*;
pub use object::*;

pub struct SceneContainer {
    pub camera: Camera,
    pub mesh: Mesh,
}

impl SceneContainer {
    pub fn new(camera: Camera, mesh: Mesh) -> Self {
        SceneContainer { camera, mesh }
    }
}

impl DynRenderObject for SceneContainer {
    fn draw(&self, _context: &mut DrawContext) {}
}
