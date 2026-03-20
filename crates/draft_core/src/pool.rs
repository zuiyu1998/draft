use std::marker::PhantomData;

use draft_arena::{Arena, Index};

pub struct Handle<T> {
    index: Index,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(index: Index) -> Self {
        Handle {
            index,
            _marker: PhantomData,
        }
    }

    pub fn slot(&self) -> u32 {
        self.index.slot
    }
}

pub struct Pool<T>(Arena<T>);

impl<T> Pool<T> {
    pub fn spawn(&mut self, value: T) -> Handle<T> {
        let index = self.0.insert(value);
        Handle {
            index,
            _marker: PhantomData,
        }
    }

    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        self.0.remove(handle.index)
    }
}
