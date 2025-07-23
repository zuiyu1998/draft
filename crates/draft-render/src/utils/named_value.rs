use std::{hash::Hash, ops::Deref};

use fyrox_core::ImmutableString;

pub struct NamedValue<T> {
    pub name: ImmutableString,
    pub value: T,
}

impl<T: PartialEq> PartialEq for NamedValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl<T: Eq> Eq for NamedValue<T> {}

impl<T: Clone> Clone for NamedValue<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Hash> Hash for NamedValue<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.value.hash(state);
    }
}

impl<T> NamedValue<T> {
    pub fn new(name: impl Into<ImmutableString>, value: T) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

impl<T> From<Vec<NamedValue<T>>> for NamedValuesContainer<T> {
    fn from(mut value: Vec<NamedValue<T>>) -> Self {
        value.sort_unstable_by_key(|prop| prop.name.cached_hash());
        Self(value)
    }
}

impl<T> Deref for NamedValuesContainer<T> {
    type Target = Vec<NamedValue<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct NamedValuesContainer<T>(Vec<NamedValue<T>>);

impl<T> NamedValuesContainer<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0.into_iter().map(|v| v.value).collect::<Vec<T>>()
    }
}

impl<T: PartialEq> PartialEq for NamedValuesContainer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Hash> Hash for NamedValuesContainer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Eq> Eq for NamedValuesContainer<T> {}

impl<T: Clone> Clone for NamedValuesContainer<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
