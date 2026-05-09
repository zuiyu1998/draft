use std::collections::HashMap;

use super::{Index, ResourceNode};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ResourceBoardKey(String);

impl<'a> From<&'a str> for ResourceBoardKey {
    fn from(s: &'a str) -> Self {
        ResourceBoardKey(String::from(s))
    }
}

#[derive(Default)]
pub struct ResourceBoard {
    resources: HashMap<ResourceBoardKey, Index<ResourceNode>>,
}

impl ResourceBoard {
    pub fn insert(&mut self, key: ResourceBoardKey, handle: Index<ResourceNode>) {
        self.resources.insert(key, handle);
    }

    pub fn get(&self, key: &ResourceBoardKey) -> Option<&Index<ResourceNode>> {
        self.resources.get(key)
    }
}
