mod payload;

pub use payload::*;

#[derive(Debug)]
pub enum ArenaError {
    InvalidIndex(u32),
    InvalidGeneration(Generation),
    InvalidType(Index),
    Empty(Index),
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
    pub(crate) slot: u32,
    pub(crate) generation: Generation,
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
