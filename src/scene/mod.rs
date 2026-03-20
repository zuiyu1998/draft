mod node;

pub use node::*;

use draft_core::pool::{Handle, Pool};

pub struct SceneGraph {
    pub root: Handle<SceneNode>,
    pub pool: Pool<SceneNode>,
    pub stack: Vec<Handle<SceneNode>>,
}

impl SceneGraph {
    pub fn empty() -> Self {
        SceneGraph {
            root: Handle::NONE,
            pool: Pool::new(),
            stack: vec![],
        }
    }

    #[inline]
    pub fn remove_node(&mut self, node_handle: Handle<SceneNode>) {
        self.isolate_node(node_handle);

        self.stack.clear();
        self.stack.push(node_handle);
        while let Some(handle) = self.stack.pop() {
            for &child in self.pool.get(handle).get_node_ref().children.iter() {
                self.stack.push(child);
            }

            self.pool.free(handle);
        }
    }

    pub fn add_node(&mut self, node: SceneNode) -> Handle<SceneNode> {
        let handle = self.pool.next_free_handle();
        self.add_node_at_handle(node, handle);
        handle
    }

    pub fn add_node_at_handle(&mut self, mut node: SceneNode, handle: Handle<SceneNode>) {
        let children = node.get_node_ref().children.clone();
        node.get_node_mut().this = handle;

        node.get_node_mut().children.clear();
        self.pool
            .insert_at_internal(handle, node)
            .expect("The handle must be valid!");

        if self.root.is_none() {
            self.root = handle;
        } else {
            self.link_nodes(handle, self.root);
        }

        for child in children {
            self.link_nodes(child, handle);
        }
    }

    #[inline]
    pub fn link_nodes(&mut self, child: Handle<SceneNode>, parent: Handle<SceneNode>) {
        self.isolate_node(child);
        self.pool.get_mut(child).get_node_mut().parent = parent;
        self.pool
            .get_mut(parent)
            .get_node_mut()
            .children
            .push(child);
    }

    #[inline]
    fn isolate_node(&mut self, node_handle: Handle<SceneNode>) {
        // Replace parent handle of child
        let parent_handle = std::mem::replace(
            &mut self.pool.get_mut(node_handle).get_node_mut().parent,
            Handle::NONE,
        );

        // Remove child from parent's children list
        if let Ok(parent) = self.pool.try_get_mut(parent_handle) {
            if let Some(i) = parent
                .get_node_ref()
                .children
                .iter()
                .position(|h| *h == node_handle)
            {
                parent.get_node_mut().children.remove(i);
            }
        }

        let (ticket, node) = self.pool.take_reserve(node_handle);
        self.pool.put_back(ticket, node);
    }
}

pub struct Scene {
    pub graph: SceneGraph,
}

#[cfg(test)]
mod tests {
    use super::{Node, SceneGraph, SceneNode};

    #[test]
    pub fn test_scene_graph() {
        let node1 = Node::empty();
        let node2 = Node::empty();
        let node3 = Node::empty();

        let mut graph = SceneGraph::empty();

        let node_handle1 = graph.add_node(SceneNode::new(node1));
        let node_handle2 = graph.add_node(SceneNode::new(node2));
        let node_handle3 = graph.add_node(SceneNode::new(node3));

        graph.link_nodes(node_handle2, node_handle1);
        graph.link_nodes(node_handle3, node_handle1);

        let len = graph.pool.len();
        assert_eq!(3, len);
        assert_eq!(graph.root, node_handle1);

        graph.remove_node(node_handle1);

        assert_eq!(graph.pool.is_valid_handle(graph.root), false);
    }
}
