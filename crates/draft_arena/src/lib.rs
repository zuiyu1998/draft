mod payload;

use std::marker::PhantomData;

pub use payload::*;

#[derive(Debug)]
pub enum ArenaError {
    InvalidIndex(u32),
    InvalidGeneration(Generation),
    InvalidType(Index),
    Empty(Index),
}

pub struct ArenaIteratorMut<'a, T> {
    ptr: *mut Entry<T>,
    end: *mut Entry<T>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ArenaIteratorMut<'a, T>
where
    T: 'static,
{
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.ptr != self.end {
                let current = &mut *self.ptr;
                if let Some(payload) = current.payload.as_mut() {
                    self.ptr = self.ptr.offset(1);
                    return Some(payload);
                }
                self.ptr = self.ptr.offset(1);
            }

            None
        }
    }
}

pub struct Arena<T> {
    storage: Vec<Entry<T>>,
    free_stack: Vec<u32>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Arena {
            storage: vec![],
            free_stack: vec![],
        }
    }
}

impl<T> Arena<T> {
    #[inline]
    pub fn free(&mut self, index: Index) -> T {
        self.try_free(index).unwrap()
    }

    #[must_use]
    #[inline]
    pub fn iter_mut(&'_ mut self) -> ArenaIteratorMut<'_, T> {
        unsafe {
            ArenaIteratorMut {
                ptr: self.storage.as_mut_ptr(),
                end: self.storage.as_mut_ptr().add(self.storage.len()),
                marker: PhantomData,
            }
        }
    }

    #[inline]
    pub fn try_free(&mut self, index: Index) -> Result<T, ArenaError> {
        let slot = usize::try_from(index.slot).expect("index overflowed usize");
        self.storage
            .get_mut(slot)
            .ok_or(ArenaError::InvalidIndex(index.slot))
            .and_then(|record| {
                if record.generation == index.generation {
                    if let Some(payload) = record.payload.take() {
                        self.free_stack.push(index.slot);
                        Ok(payload)
                    } else {
                        Err(ArenaError::Empty(index))
                    }
                } else {
                    Err(ArenaError::InvalidGeneration(index.generation))
                }
            })
    }

    #[inline]
    pub fn put_back(&mut self, ticket: Ticket, value: T) -> Index {
        let entry = self
            .get_mut_by_slot(ticket.index)
            .expect("Ticket index was invalid");
        let old = entry.payload.replace(value);
        assert!(old.is_none());
        let index = Index::new(ticket.index, entry.generation);
        std::mem::forget(ticket);
        index
    }

    pub fn take_reserve(&mut self, index: Index) -> (Ticket, T) {
        self.try_take_reserve(index).unwrap()
    }

    #[inline]
    pub fn try_take_reserve(&mut self, index: Index) -> Result<(Ticket, T), ArenaError> {
        let entry = self.get_mut_by_slot(index.slot)?;
        if entry.generation == index.generation {
            if let Some(payload) = entry.payload.take() {
                let ticket = Ticket { index: index.slot };
                Ok((ticket, payload))
            } else {
                Err(ArenaError::Empty(index))
            }
        } else {
            Err(ArenaError::InvalidGeneration(index.generation))
        }
    }

    pub fn try_borrow_mut(&mut self, index: Index) -> Result<&mut T, ArenaError> {
        self.get_mut_by_slot(index.slot).and_then(|r| {
            if r.generation == index.generation {
                r.payload.as_mut().ok_or(ArenaError::Empty(index))
            } else {
                Err(ArenaError::InvalidGeneration(index.generation))
            }
        })
    }

    pub fn try_borrow(&self, index: Index) -> Result<&T, ArenaError> {
        self.get_by_slot(index.slot).and_then(|r| {
            if r.generation == index.generation {
                r.payload.as_ref().ok_or(ArenaError::Empty(index))
            } else {
                Err(ArenaError::InvalidGeneration(index.generation))
            }
        })
    }

    pub fn get_mut_by_slot(&mut self, slot: u32) -> Result<&mut Entry<T>, ArenaError> {
        self.storage
            .get_mut(usize::try_from(slot).expect("Index overflowed usize"))
            .ok_or(ArenaError::InvalidIndex(slot))
    }

    pub fn get_by_slot(&self, slot: u32) -> Result<&Entry<T>, ArenaError> {
        self.storage
            .get(usize::try_from(slot).expect("Index overflowed usize"))
            .ok_or(ArenaError::InvalidIndex(slot))
    }

    pub fn next_free_index(&self) -> Index {
        if let Some(index) = self.free_stack.last().cloned() {
            let generation = self.storage[index as usize].generation.next();
            Index::new(index, generation)
        } else {
            Index::new(self.storage.len() as u32, Generation::first())
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn insert_at_internal(&mut self, index: Index, payload: T) -> Result<Index, T> {
        match self.storage.get_mut(index.slot as usize) {
            Some(record) => match record.payload.as_ref() {
                Some(_) => Err(payload),
                None => {
                    let position = self
                        .free_stack
                        .iter()
                        .rposition(|i| *i == index.slot)
                        .expect("free_stack must contain the index of the empty record (most likely attempting to spawn at a reserved index)!");

                    self.free_stack.remove(position);

                    let generation = if !index.is_viald() {
                        record.generation.next()
                    } else {
                        index.generation
                    };

                    record.generation = generation;
                    record.payload = Payload::new(payload);

                    Ok(Index {
                        slot: index.slot,
                        generation,
                    })
                }
            },
            None => {
                // Spawn missing records to fill gaps.
                for i in self.len()..index.slot as usize {
                    self.storage.push(Entry {
                        generation: Generation::first(),
                        payload: Payload::new_empty(),
                    });
                    self.free_stack.push(i as u32);
                }

                let generation = if !index.is_viald() {
                    Generation::first()
                } else {
                    index.generation
                };

                self.storage.push(Entry {
                    generation,
                    payload: Payload::new(payload),
                });

                Ok(Index::new(index.slot, generation))
            }
        }
    }

    pub fn is_valid_handle(&self, index: Index) -> bool {
        if let Some(record) = self.storage.get(index.slot as usize) {
            record.payload.is_some() && record.generation == index.generation
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: Index) -> Option<T> {
        self.try_remove(index).ok()
    }

    #[inline]
    pub fn try_remove(&mut self, index: Index) -> Result<T, ArenaError> {
        let slot = usize::try_from(index.slot).expect("index overflowed usize");
        self.storage
            .get_mut(slot)
            .ok_or(ArenaError::InvalidIndex(index.slot))
            .and_then(|entry: &mut Entry<T>| {
                if entry.generation == index.generation {
                    if let Some(payload) = entry.payload.take() {
                        self.free_stack.push(index.slot);
                        Ok(payload)
                    } else {
                        Err(ArenaError::Empty(index))
                    }
                } else {
                    Err(ArenaError::InvalidGeneration(index.generation))
                }
            })
    }

    pub fn insert(&mut self, value: T) -> Index {
        self.insert_with(|_| value)
    }

    pub fn insert_with<F: FnOnce(Index) -> T>(&mut self, callback: F) -> Index {
        if let Some(free_index) = self.free_stack.pop() {
            let entry = self
                .storage
                .get_mut(free_index as usize)
                .expect("free stack contained invalid index");

            if entry.payload.is_some() {
                panic!(
                    "Attempt to spawn an object at pool record with payload! Record index is {free_index}"
                );
            }

            let generation = entry.generation.next();
            let index = Index {
                slot: free_index,
                generation,
            };

            let payload = callback(index);

            entry.generation = generation;
            entry.payload.replace(payload);
            index
        } else {
            // No free records, create new one
            let generation = Generation::first();

            let handle = Index {
                slot: self.storage.len() as u32,
                generation,
            };

            let payload = callback(handle);

            let record = Entry {
                generation,
                payload: Payload::new(payload),
            };

            self.storage.push(record);

            handle
        }
    }
}

#[derive(Debug)]
pub struct Ticket {
    index: u32,
}

impl Drop for Ticket {
    fn drop(&mut self) {
        panic!(
            "An object at index {} must be returned to a arena it was taken from! \
            Call Arena::forget_ticket if you don't need the object anymore.",
            self.index
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(u32);

impl Generation {
    pub const INVIALD: Generation = Generation(0);

    #[must_use]
    pub(crate) fn first() -> Self {
        Generation(1)
    }

    #[must_use]
    pub(crate) fn next(&self) -> Self {
        Generation(self.0 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Index {
    pub slot: u32,
    pub generation: Generation,
}

impl Index {
    pub const INVIALD: Index = Index {
        slot: 0,
        generation: Generation::INVIALD,
    };

    pub fn new(slot: u32, generation: Generation) -> Self {
        Self { slot, generation }
    }

    pub fn is_viald(&self) -> bool {
        !(*self == Self::INVIALD)
    }
}

pub struct Entry<T> {
    generation: Generation,
    payload: Payload<Option<T>>,
}

#[cfg(test)]
mod tests {
    use crate::Arena;

    #[test]
    fn test_area() {
        let mut area = Arena::default();
        let index = area.insert(1);

        let value = area.remove(index).unwrap();

        assert_eq!(value, 1);
    }
}
