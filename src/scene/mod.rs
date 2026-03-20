mod node;

pub use node::*;

use draft_core::pool::{Handle, Pool};

pub struct SceneGraph {
    pub root: Handle<SceneNode>,
    pub pool: Pool<SceneNode>,
}

impl SceneGraph {
    pub fn add_node(&mut self, mut _node: SceneNode) -> Handle<SceneNode> {
        todo!()
    }
}

pub struct Scene {
    pub graph: SceneGraph,
}
