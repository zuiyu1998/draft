use crate::frame_graph::{Handle, Pass, RawResourceHandle, ResourceNode};

pub struct PassNode {
    pub name: String,
    pub handle: Handle<PassNode>,
    pub writes: Vec<RawResourceHandle>,
    pub reads: Vec<RawResourceHandle>,
    pub resource_request_array: Vec<Handle<ResourceNode>>,
    pub resource_release_array: Vec<Handle<ResourceNode>>,
    pub pass: Option<Pass>,
}

impl PassNode {
    pub fn new(name: &str, handle: Handle<PassNode>) -> Self {
        Self {
            name: name.to_string(),
            handle,
            writes: Default::default(),
            reads: Default::default(),
            resource_request_array: Default::default(),
            resource_release_array: Default::default(),
            pass: Default::default(),
        }
    }
}
