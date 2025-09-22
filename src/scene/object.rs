use downcast_rs::{Downcast, impl_downcast};

use crate::renderer::{MeshRenderDataBundleStorage, Observer};

pub trait DynSceneObject: 'static + Downcast {}

pub struct SceneObject(Box<dyn DynSceneObject>);

impl SceneObject {
    pub fn new<T: DynSceneObject>(value: T) -> Self {
        SceneObject(Box::new(value))
    }

    pub fn cast<T: DynSceneObject>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }

    pub fn cast_mut<T: DynSceneObject>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut()
    }
}

impl_downcast!(DynSceneObject);

pub struct DrawContext<'a> {
    pub mesh_render_data_bundle_storage: &'a mut dyn MeshRenderDataBundleStorage,
    pub cameras: &'a mut Vec<Observer>,
}

pub trait DynRenderObject {
    fn draw(&self, _context: &mut DrawContext) {}
}
