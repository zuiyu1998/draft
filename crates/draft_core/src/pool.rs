use std::{cmp::Ordering, marker::PhantomData};

use draft_arena::{Arena, Index};

pub struct Handle<T> {
    index: Index,
    _marker: PhantomData<T>,
}

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            index: self.index,
            _marker: PhantomData,
        }
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Ord for Handle<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> PartialEq<Self> for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> PartialOrd for Handle<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Handle<T> {
    pub const INVIALD: Handle<T> = Handle {
        index: Index::INVIALD,
        _marker: PhantomData,
    };

    pub fn new(index: Index) -> Self {
        Handle {
            index,
            _marker: PhantomData,
        }
    }

    pub fn slot(&self) -> u32 {
        self.index.slot
    }

    pub fn is_inviald(&self) -> bool {
        *self == Self::INVIALD
    }

    pub fn is_viald(&self) -> bool {
        !self.is_inviald()
    }
}

pub struct Pool<T>(Arena<T>);

impl<T> Pool<T> {
    pub fn is_valid_handle(&self, handle: Handle<T>) -> bool {
        self.0.is_valid_handle(handle.index)
    }

    pub fn spawn(&mut self, value: T) -> Handle<T> {
        let index = self.0.insert(value);
        Handle::new(index)
    }

    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        self.0.remove(handle.index)
    }
}
