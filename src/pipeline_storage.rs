use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use frame_graph::{
    GetPipelineCache, PipelineCache, RenderDevice,
    wgpu::{self, ShaderModuleDescriptor, ShaderSource},
};
use fyrox_core::{
    log::Log,
    sparse::{AtomicIndex, SparseBuffer},
};
use fyrox_resource::entry::DEFAULT_RESOURCE_LIFETIME;
use thiserror::Error;

use crate::{Shader, ShaderResource};

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
    pub self_index: Arc<AtomicIndex>,
}

impl<T> Drop for CacheEntry<T> {
    fn drop(&mut self) {
        // Reset self index to unassigned. This is needed, because there could be the following
        // situation:
        // 1) Cache entry was removed
        // 2) Its index was stored somewhere else.
        // 3) The index can then be used to access some entry on the index, but the cache cannot
        // guarantee, that the data of the entry is the same.
        self.self_index.set(AtomicIndex::UNASSIGNED_INDEX)
    }
}

impl<T> Deref for CacheEntry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for CacheEntry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub struct TemporaryCache<T> {
    pub buffer: SparseBuffer<CacheEntry<T>>,
}

impl<T> Default for TemporaryCache<T> {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
        }
    }
}

impl<T> TemporaryCache<T> {
    pub fn spawn(
        &mut self,
        value: T,
        self_index: Arc<AtomicIndex>,
        time_to_live: TimeToLive,
    ) -> AtomicIndex {
        let index = self.buffer.spawn(CacheEntry {
            value,
            time_to_live,
            self_index,
        });

        self.buffer
            .get_mut(&index)
            .unwrap()
            .self_index
            .set(index.get());

        index
    }

    pub fn get_mut(&mut self, index: &AtomicIndex) -> Option<&mut CacheEntry<T>> {
        if let Some(entry) = self.buffer.get_mut(index) {
            entry.time_to_live = TimeToLive::default();
            Some(entry)
        } else {
            None
        }
    }

    pub fn get_entry_mut_or_insert_with<F, E>(
        &mut self,
        index: &Arc<AtomicIndex>,
        time_to_live: TimeToLive,
        func: F,
    ) -> Result<&mut CacheEntry<T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if let Some(entry) = self.buffer.get_mut(index) {
            entry.time_to_live = time_to_live;
            Ok(self.buffer.get_mut(index).unwrap())
        } else {
            let value = func()?;
            let index = self.buffer.spawn(CacheEntry {
                value,
                time_to_live,
                self_index: index.clone(),
            });
            let entry = self.buffer.get_mut(&index).unwrap();
            entry.self_index.set(index.get());
            Ok(entry)
        }
    }

    pub fn get_mut_or_insert_with<F, E>(
        &mut self,
        index: &Arc<AtomicIndex>,
        time_to_live: TimeToLive,
        func: F,
    ) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.get_entry_mut_or_insert_with(index, time_to_live, func)
            .map(|entry| &mut entry.value)
    }

    pub fn get_or_insert_with<F, E>(
        &mut self,
        index: &Arc<AtomicIndex>,
        time_to_live: TimeToLive,
        func: F,
    ) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        self.get_entry_mut_or_insert_with(index, time_to_live, func)
            .map(|entry| &entry.value)
    }

    pub fn update(&mut self, dt: f32) {
        for entry in self.buffer.iter_mut() {
            *entry.time_to_live -= dt;
        }

        for i in 0..self.buffer.len() {
            if let Some(entry) = self.buffer.get_raw(i) {
                if *entry.time_to_live <= 0.0 {
                    self.buffer.free_raw(i);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn alive_count(&self) -> usize {
        self.buffer.filled()
    }

    pub fn remove(&mut self, index: &AtomicIndex) {
        self.buffer.free(index);
    }
}

pub struct ShaderModuleData {
    pub module: Arc<wgpu::ShaderModule>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
}

impl ShaderModuleData {
    pub fn new(
        composer: &mut naga_oil::compose::Composer,
        device: &RenderDevice,
        shader: &Shader,
    ) -> Result<Self, Error> {
        let naga = composer.make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            ..(&shader.definition).into()
        })?;

        let shader_source = ShaderSource::Naga(Cow::Owned(naga));

        let module_descriptor = ShaderModuleDescriptor {
            label: None,
            source: shader_source,
        };

        let shader_module = device.wgpu_device().create_shader_module(module_descriptor);

        Ok(ShaderModuleData {
            module: Arc::new(shader_module),
        })
    }
}

pub struct ShaderCache {
    composer: naga_oil::compose::Composer,
    cache: TemporaryCache<ShaderModuleData>,
}

impl ShaderCache {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        shader: &ShaderResource,
    ) -> Option<Arc<wgpu::ShaderModule>> {
        let mut shader_state = shader.state();

        if let Some(shader_state) = shader_state.data() {
            match self.cache.get_or_insert_with(
                &shader_state.cache_index,
                Default::default(),
                || ShaderModuleData::new(&mut self.composer, device, shader_state),
            ) {
                Ok(data) => Some(data.module.clone()),
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}

pub struct PipelineStorage {}

impl GetPipelineCache for PipelineStorage {
    fn get_pipeline_cache(&self) -> PipelineCache {
        todo!()
    }
}
