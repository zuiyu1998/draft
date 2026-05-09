use crate::frame_graph::{Index, Pass, RawResourceHandle, ResourceNode};

pub struct PassNode {
    pub name: String,
    pub index: Index<PassNode>,
    pub writes: Vec<RawResourceHandle>,
    pub reads: Vec<RawResourceHandle>,
    pub resource_request_array: Vec<Index<ResourceNode>>,
    pub resource_release_array: Vec<Index<ResourceNode>>,
    pub pass: Option<Pass>,
}

impl PassNode {
    pub fn new(name: &str, index: Index<PassNode>) -> Self {
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
