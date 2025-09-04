mod uniform_buffer;

pub use uniform_buffer::*;

use std::sync::Arc;

use crate::{
    FrameworkError, TemporaryCache, frame_graph::BufferInfo, render_resource::RenderBuffer,
};
use draft_gfx_base::{GpuBuffer, RenderDevice, RenderQueue};
use fxhash::FxHashMap;
use fyrox_core::sparse::AtomicIndex;

pub struct BufferKey {
    desc: BufferInfo,
    index: Arc<AtomicIndex>,
}

impl BufferKey {
    pub fn get_render_buffer_key(&self) -> String {
        match &self.desc.label {
            Some(label) => {
                format!("buffer_{}_{}_{}", label, self.desc.size, self.index.get())
            }
            _ => {
                format!("buffer_{}_{}", self.desc.size, self.index.get())
            }
        }
    }
}

pub struct BufferSet {
    cache: TemporaryCache<GpuBuffer>,
    free: usize,
}

impl Default for BufferSet {
    fn default() -> Self {
        Self::new()
    }
}

fn create_buffer(device: &RenderDevice, desc: &BufferInfo) -> Result<GpuBuffer, FrameworkError> {
    Ok(device.create_buffer(&desc.to_buffer_desc()))
}

impl BufferSet {
    pub fn new() -> Self {
        BufferSet {
            cache: Default::default(),
            free: 0,
        }
    }

    fn mark_unused(&mut self) {
        self.free = 0;
    }

    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get_render_buffer(&self, key: &BufferKey) -> Option<RenderBuffer> {
        self.cache.buffer.get(&key.index).map(|entry| RenderBuffer {
            key: key.get_render_buffer_key(),
            value: entry.value.clone(),
            desc: key.desc.clone(),
        })
    }

    fn get_or_create(&mut self, device: &RenderDevice, desc: &BufferInfo) -> BufferKey {
        let last_free = self.free;
        let index: Arc<AtomicIndex> = Default::default();
        index.set(last_free);

        let _ = self
            .cache
            .get_or_insert_with(&index, Default::default(), || create_buffer(device, desc));

        self.free += 1;

        BufferKey {
            desc: desc.clone(),
            index,
        }
    }
}

pub struct BufferCache {
    device: RenderDevice,
    queue: RenderQueue,
    cache: FxHashMap<BufferInfo, BufferSet>,
}

impl BufferCache {
    pub fn new(device: RenderDevice, queue: RenderQueue) -> Self {
        Self {
            device,
            cache: Default::default(),
            queue,
        }
    }

    pub fn get_render_buffer(&self, key: &BufferKey) -> RenderBuffer {
        self.try_get_render_buffer(key).expect("must have  buffer")
    }

    pub fn try_get_render_buffer(&self, key: &BufferKey) -> Option<RenderBuffer> {
        self.cache
            .get(&key.desc)
            .and_then(|set| set.get_render_buffer(key))
    }

    pub fn upload_bytes(&mut self, key: &BufferKey, bytes: &[u8]) {
        if let Some(set) = self.cache.get_mut(&key.desc) {
            if let Some(render_buffer) = set.get_render_buffer(key) {
                self.queue
                    .wgpu_queue()
                    .write_buffer(render_buffer.value.get_buffer(), 0, bytes);
            }
        }
    }

    pub fn get_or_create(&mut self, desc: &BufferInfo) -> BufferKey {
        let set = self.cache.entry(desc.clone()).or_default();
        set.get_or_create(&self.device, desc)
    }

    pub fn mark_all_unused(&mut self) {
        for set in self.cache.values_mut() {
            set.mark_unused();
        }
    }

    pub fn update(&mut self, dt: f32) {
        for set in self.cache.values_mut() {
            set.update(dt);
        }
    }
}
