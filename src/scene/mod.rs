pub mod camera;
pub mod mesh;
pub mod node;
pub mod object;

pub use camera::*;
pub use mesh::*;
pub use node::*;
pub use object::*;

use crate::renderer::{ObserversCollection, RenderDataBundleStorage};

pub struct SceneContainer {
    pub camera: Camera,
    pub mesh: Mesh,
}

impl SceneContainer {
    pub fn new(camera: Camera, mesh: Mesh) -> Self {
        SceneContainer { camera, mesh }
    }
}

pub struct DrawContext<'a> {
    pub render_data_bundle_storage: &'a mut dyn RenderDataBundleStorage,
    pub observers_collection: &'a mut ObserversCollection,
}

impl DrawContext<'_> {
    pub fn collect_render_data(&mut self, scene_container: &SceneContainer) {
        scene_container.camera.draw(self);
        scene_container.mesh.draw(self);
    }
}
