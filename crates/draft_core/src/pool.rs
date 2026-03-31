use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use draft_arena::{Arena, ArenaError, Index, Ticket};

#[derive(Debug)]
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

impl<T> Hash for Handle<T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.index.hash(state);
    }
}

impl<T> Handle<T> {
    pub const NONE: Handle<T> = Handle {
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

    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
}

pub struct Pool<T>(Arena<T>);

impl<T: 'static> Pool<T> {
    pub fn new() -> Self {
        Pool(Arena::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn free(&mut self, handle: Handle<T>) -> T {
        self.0.free(handle.index)
    }

    pub fn put_back(&mut self, ticket: Ticket, value: T) -> Handle<T> {
        let index = self.0.put_back(ticket, value);
        Handle::new(index)
    }

    pub fn take_reserve(&mut self, handle: Handle<T>) -> (Ticket, T) {
        self.0.take_reserve(handle.index)
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T {
        self.try_get_mut(handle).unwrap()
    }

    pub fn try_get_mut(&mut self, handle: Handle<T>) -> Result<&mut T, ArenaError> {
        self.0.try_borrow_mut(handle.index)
    }

    pub fn get(&self, handle: Handle<T>) -> &T {
        self.0.try_borrow(handle.index).unwrap()
    }

    pub fn next_free_handle(&mut self) -> Handle<T> {
        let index = self.0.next_free_index();
        Handle::new(index)
    }

    pub fn insert_at_internal(&mut self, handle: Handle<T>, payload: T) -> Result<Handle<T>, T> {
        self.0
            .insert_at_internal(handle.index, payload)
            .map(|index| Handle::new(index))
    }

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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}
