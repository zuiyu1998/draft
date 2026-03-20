mod node;

pub use node::*;

use draft_core::pool::{Handle, Pool};

pub struct SceneGraph {
    pub root: Handle<SceneNode>,
    pub pool: Pool<SceneNode>,
}

impl SceneGraph {
    pub fn add_node(&mut self, node: SceneNode) -> Handle<SceneNode> {
        let handle = self.pool.next_free_handle();
        self.add_node_at_handle(node, handle);
        handle
    }

    pub fn add_node_at_handle(&mut self, mut node: SceneNode, handle: Handle<SceneNode>) {
        let children = node.get_node_ref().children.clone();
        node.get_node_mut().children.clear();
        self.pool
            .insert_at_internal(handle, node)
            .expect("The handle must be valid!");

        if self.root.is_inviald() {
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
            Handle::INVIALD,
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
