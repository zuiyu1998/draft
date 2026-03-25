use std::collections::HashMap;

use super::{Handle, ResourceNode};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ResourceBoardKey(String);

impl<'a> From<&'a str> for ResourceBoardKey {
    fn from(s: &'a str) -> Self {
        ResourceBoardKey(String::from(s))
    }
}

#[derive(Default)]
pub struct ResourceBoard {
    resources: HashMap<ResourceBoardKey, Handle<ResourceNode>>,
}

impl ResourceBoard {
    pub fn insert(&mut self, key: ResourceBoardKey, handle: Handle<ResourceNode>) {
        self.resources.insert(key, handle);
    }

    pub fn get(&self, key: &ResourceBoardKey) -> Option<&Handle<ResourceNode>> {
        self.resources.get(key)
    }
}
