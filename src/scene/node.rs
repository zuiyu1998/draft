use std::fmt::Debug;

use downcast_rs::Downcast;
use draft_core::pool::Handle;

///场景节点共有的数据
pub struct Node {
    pub this: Handle<SceneNode>,
    pub parent: Handle<SceneNode>,
    pub children: Vec<Handle<SceneNode>>,
}

/// 场景节点
#[derive(Debug)]
pub struct SceneNode(Box<dyn SceneNodeBehavior>);

impl Clone for SceneNode {
    fn clone(&self) -> Self {
        SceneNode(self.0.clone_boxed())
    }
}

impl SceneNode {
    pub fn new<T: SceneNodeBehavior>(value: T) -> Self {
        SceneNode(Box::new(value))
    }

    pub fn get_node_ref(&self) -> &Node {
        self.0.get_node_ref()
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        self.0.get_node_mut()
    }
}

pub trait SceneNodeBehavior: Downcast + Debug {
    fn get_node_ref(&self) -> &Node;

    fn get_node_mut(&self) -> &mut Node;

    fn clone_boxed(&self) -> Box<dyn SceneNodeBehavior>;
}
