use fyrox_core::ImmutableString;
use fyrox_core::{reflect::*, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, PartialEq, Eq, Hash)]
pub enum ResourceKey {
    BuiltIn(ImmutableString),
    Local(ImmutableString),
}

impl Default for ResourceKey {
    fn default() -> Self {
        Self::Local(ImmutableString::new(""))
    }
}

impl ResourceKey {
    pub fn immutable_string(&self) -> &ImmutableString {
        match self {
            ResourceKey::BuiltIn(v) => v,
            ResourceKey::Local(v) => v,
        }
    }

    pub fn is_built_in(&self) -> bool {
        matches!(self, ResourceKey::BuiltIn(_))
    }

    pub fn new_local(name: &str) -> Self {
        Self::Local(ImmutableString::new(name))
    }

    pub fn new_built_in(name: &str) -> Self {
        Self::BuiltIn(ImmutableString::new(name))
    }
}
