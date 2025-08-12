use fxhash::FxHashMap;
use fyrox_core::{reflect::*, visitor::*};

use crate::{MaterialResourceBinding, ResourceBindingName};

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ResourceBindings(FxHashMap<ResourceBindingName, MaterialResourceBinding>);

impl ResourceBindings {
    pub fn get(&self, key: &ResourceBindingName) -> Option<&MaterialResourceBinding> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: ResourceBindingName, binding: MaterialResourceBinding) {
        self.0.insert(key, binding);
    }
}
