mod builder;

pub use builder::*;

use draft_render::RenderDataBundleStorage;
use fyrox_core::{
    algebra::{Matrix4, Vector3},
    math::Matrix4Ext,
};
use std::cell::Cell;

use crate::{renderer::ObserversCollection, scene::DynSceneObject};

#[derive(Default)]
pub struct Node {
    pub(crate) global_transform: Cell<Matrix4<f32>>,
}

impl DynSceneObject for Node {}

impl DynSceneNode for Node {
    fn get_ref(&self) -> NodeRef {
        NodeRef {
            global_transform: self.global_transform.get(),
        }
    }

    fn get_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut {
            global_transform: &mut self.global_transform,
        }
    }
}

pub struct NodeRef {
    global_transform: Matrix4<f32>,
}

impl NodeRef {
    pub fn look_vector(&self) -> Vector3<f32> {
        self.global_transform.look()
    }

    pub fn global_position(&self) -> Vector3<f32> {
        self.global_transform.position()
    }

    pub fn up_vector(&self) -> Vector3<f32> {
        self.global_transform.up()
    }
}

pub struct NodeMut<'a> {
    pub global_transform: &'a mut Cell<Matrix4<f32>>,
}

pub struct NodeContext<'a> {
    pub render_data_bundle_storage: &'a mut dyn RenderDataBundleStorage,
    pub observers_collection: &'a mut ObserversCollection,
}

pub trait DynSceneNode: DynSceneObject {
    fn get_ref(&self) -> NodeRef;

    fn get_mut<'a>(&'a mut self) -> NodeMut<'a>;

    fn collect_render_data(&self, _context: &mut NodeContext) {}
}
