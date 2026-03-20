use downcast_rs::Downcast;

///场景节点共有的数据
pub struct Node {}

/// 场景节点
pub struct SceneNode(Box<dyn SceneNodeBehavior>);

impl SceneNode {
    pub fn new<T: SceneNodeBehavior>(value: T) -> Self {
        SceneNode(Box::new(value))
    }

    pub fn get_node_ref(&self) -> &Node {
        self.0.get_node_ref()
    }

    pub fn get_node_mut(&mut self) -> &Node {
        self.0.get_node_mut()
    }
}

pub trait SceneNodeBehavior: Downcast {
    fn get_node_ref(&self) -> &Node;

    fn get_node_mut(&self) -> &mut Node;
}
