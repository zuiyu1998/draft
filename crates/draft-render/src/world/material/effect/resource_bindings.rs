use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};

use crate::MaterialResourceBinding;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ResourceBindings(FxHashMap<ImmutableString, MaterialResourceBinding>);

impl ResourceBindings {
    pub fn get(&self, key: &ImmutableString) -> Option<&MaterialResourceBinding> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: ImmutableString, binding: MaterialResourceBinding) {
        self.0.insert(key, binding);
    }
}
