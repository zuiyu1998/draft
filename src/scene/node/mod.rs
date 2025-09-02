mod builder;

pub use builder::*;

use downcast_rs::{Downcast, impl_downcast};
use fyrox_core::{
    algebra::{Matrix4, Vector3},
    math::Matrix4Ext,
};
use std::cell::Cell;

pub struct AbstractNode(Box<dyn DynNode>);

impl AbstractNode {
    pub fn new<T: DynNode>(value: T) -> Self {
        AbstractNode(Box::new(value))
    }

    pub fn cast<T: DynNode>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }

    pub fn cast_mut<T: DynNode>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut()
    }
}

pub struct Node {
    pub(crate) global_transform: Cell<Matrix4<f32>>,
}

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

pub trait DynNode: 'static + Downcast {
    fn get_ref(&self) -> NodeRef;

    fn get_mut(&mut self) -> NodeMut;
}

impl_downcast!(DynNode);
