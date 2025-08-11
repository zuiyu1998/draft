mod buffer_allocator;

pub use buffer_allocator::*;

use crate::{
    TemporaryCache,
    frame_graph::BufferInfo,
    gfx_base::{RawBuffer, RenderDevice, RenderQueue},
    render_resource::RenderBuffer,
};
use fxhash::FxHashMap;
use fyrox_core::sparse::AtomicIndex;

pub struct BufferKey {
    desc: BufferInfo,
    index: AtomicIndex,
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
    cache: TemporaryCache<RawBuffer>,
    free: usize,
}

impl Default for BufferSet {
    fn default() -> Self {
        Self::new()
    }
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
        let index = if self.free < self.cache.buffer.len() {
            let last_free = self.free;
            self.free += 1;

            let index: AtomicIndex = Default::default();
            index.set(last_free);

            index
        } else {
            let buffer = device.wgpu_device().create_buffer(&desc.get_buffer_desc());

            let index = self
                .cache
                .spawn(buffer, Default::default(), Default::default());

            self.free = self.cache.buffer.len();
            index
        };

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
                    .write_buffer(&render_buffer.value, 0, bytes);
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
