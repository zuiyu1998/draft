mod builder;

pub use builder::*;

use fyrox_core::{
    algebra::{Matrix4, Vector3},
    math::Matrix4Ext,
};
use std::cell::Cell;

use crate::scene::DynObject;

pub struct Node {
    pub(crate) global_transform: Cell<Matrix4<f32>>,
}

impl DynObject for Node {}

impl DynNode for Node {
    fn get_ref(&self) -> NodeRef {
        NodeRef {
            global_transform: self.global_transform.get(),
        }
    }

    fn get_mut(&mut self) -> NodeMut {
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

pub trait DynNode: DynObject {
    fn get_ref(&self) -> NodeRef;

    fn get_mut(&mut self) -> NodeMut;
}
