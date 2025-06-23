use crate::frame_graph::Pass;

use super::{GraphRawResourceHandle, IndexHandle, ResourceNode};

pub struct PassNode {
    pub name: String,
    pub index: IndexHandle<PassNode>,
    pub writes: Vec<GraphRawResourceHandle>,
    pub reads: Vec<GraphRawResourceHandle>,
    pub resource_request_array: Vec<IndexHandle<ResourceNode>>,
    pub resource_release_array: Vec<IndexHandle<ResourceNode>>,
    pub pass: Option<Pass>,
}

impl PassNode {
    pub fn new(name: &str, index: IndexHandle<PassNode>) -> Self {
        Self {
            name: name.to_string(),
            index,
            writes: Default::default(),
            reads: Default::default(),
            resource_request_array: Default::default(),
            resource_release_array: Default::default(),
            pass: Default::default(),
        }
    }
}
