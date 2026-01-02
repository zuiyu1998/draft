mod material_effect_cache;
mod mesh_cache;

pub use material_effect_cache::*;
pub use mesh_cache::*;

use fyrox_resource::entry::DEFAULT_RESOURCE_LIFETIME;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq)]
pub struct TimeToLive(pub f32);

impl Default for TimeToLive {
    fn default() -> Self {
        Self(DEFAULT_RESOURCE_LIFETIME)
    }
}

impl Deref for TimeToLive {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimeToLive {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct CacheEntry<T> {
    pub value: T,
    pub time_to_live: TimeToLive,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            time_to_live: Default::default(),
        }
    }

    fn update(&mut self, dt: f32) {
        *self.time_to_live -= dt;
    }

    fn free(&self) -> bool {
        self.time_to_live.0 <= 0.0
    }
}
