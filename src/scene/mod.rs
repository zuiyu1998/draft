pub mod camera;
pub mod mesh;
pub mod node;
pub mod object;

pub use camera::*;
pub use mesh::*;
pub use node::*;
pub use object::*;

#[derive(Default)]
pub struct SceneContainer {
    pub cameras: Vec<Camera>,
    pub meshs: Vec<Mesh>,
}

impl DynRenderObject for SceneContainer {
    fn draw(&self, context: &mut DrawContext) {
        for camera in self.cameras.iter() {
            camera.draw(context);
        }

        for mesh in self.meshs.iter() {
            mesh.draw(context);
        }
    }
}
