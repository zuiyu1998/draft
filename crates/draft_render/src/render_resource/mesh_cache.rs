use std::{
    collections::{HashSet, hash_map::Entry},
    ops::{Deref, DerefMut},
};

use draft_graphics::gfx_base::{RenderDevice, RenderQueue};
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use fxhash::FxHashMap;
use fyrox_resource::entry::DEFAULT_RESOURCE_LIFETIME;

use crate::{BufferAllocator, MeshAllocator, MeshAllocatorSettings};

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

pub struct MeshData {
    key: u64,
    modifications_counter: u64,
}

#[derive(Default)]
pub struct MeshCache {
    cache: FxHashMap<u64, CacheEntry<MeshData>>,
    data: FxHashMap<u64, MeshResource>,

    removed: HashSet<u64>,
    modified: HashSet<u64>,
    added: HashSet<u64>,
}

impl MeshCache {
    pub fn insert_mesh(&mut self, mesh: &MeshResource) {
        let key = mesh.key();

        self.data.insert(key, mesh.clone());
        self.removed.remove(&key);

        let mesh = mesh.data_ref();

        match self.cache.entry(key) {
            Entry::Occupied(mut entry) => {
                if entry.get().value.modifications_counter != mesh.modifications_counter() {
                    entry.get_mut().value.modifications_counter = mesh.modifications_counter();
                    self.modified.insert(key);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(CacheEntry::new(MeshData {
                    key,
                    modifications_counter: mesh.modifications_counter(),
                }));

                self.added.insert(key);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for data in self.cache.values_mut() {
            data.update(dt);
            if data.free() {
                self.removed.insert(data.value.key);
            }
        }
    }

    pub fn allocate_and_free_meshes(
        &mut self,
        settings: &MeshAllocatorSettings,
        layouts: &mut MeshVertexBufferLayouts,
        buffer_allocator: &mut BufferAllocator,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
        mesh_allocator: &mut MeshAllocator,
    ) {
        let meshes_to_free = self.removed.iter().chain(self.modified.iter());
        mesh_allocator.free_meshes(meshes_to_free);

        let mut added = vec![];

        for key in self.added.iter() {
            if let Some(mesh) = self.data.get(key) {
                added.push(mesh.clone());
            }
        }

        mesh_allocator.allocate_meshes(
            &added,
            settings,
            layouts,
            buffer_allocator,
            render_device,
            render_queue,
        );
    }
}
