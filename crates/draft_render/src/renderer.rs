use std::collections::{HashMap, hash_map::IterMut};

pub trait MeshMaterialRenderer: 'static {}

pub struct MeshMaterialRenderer2d {}

impl MeshMaterialRenderer for MeshMaterialRenderer2d {}

#[derive(Default)]
pub struct MeshMaterialRendererContainer {
    renderers: HashMap<String, Box<dyn MeshMaterialRenderer>>,
}

impl MeshMaterialRendererContainer {
    pub fn insert(&mut self, key: &str, renderer: impl MeshMaterialRenderer) {
        self.insert_boxed(key, Box::new(renderer));
    }

    fn insert_boxed(&mut self, key: &str, renderer: Box<dyn MeshMaterialRenderer>) {
        self.renderers.insert(key.to_string(), renderer);
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, String, Box<dyn MeshMaterialRenderer + 'static>> {
        self.renderers.iter_mut()
    }
}
