use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub struct Handle<T> {
    pub index: usize,
    _marker: PhantomData<T>,
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Handle<T> {
    pub fn new(index: usize) -> Self {
        Handle {
            index,
            _marker: PhantomData,
        }
    }
}
