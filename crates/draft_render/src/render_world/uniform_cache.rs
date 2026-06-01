use std::collections::HashMap;

use crate::render_world::TemporaryCache;
use draft_graphics::{Buffer, BufferDescriptor, BufferUsages, RenderDevice, RenderQueue};

pub struct UniformRenderData {
    buffer: Buffer,
}

#[derive(Default)]
pub struct UniformSet {
    cache: TemporaryCache<UniformRenderData>,
    pointer: usize,
}

impl UniformSet {
    pub fn unset(&mut self) {
        self.pointer = 0;
    }

    pub fn get_or_create(
        &mut self,
        device: &RenderDevice,
        queue: &RenderQueue,
        name: &str,
        bytes: &[u8],
        usage: BufferUsages,
    ) -> usize {
        let index;

        if self.pointer >= self.cache.buffer.len() {
            index = self.cache.buffer.len();

            let buffer = device.create_gpu_buffer(&BufferDescriptor {
                label: Some(&format!("Uniform {} {}", name, index)),
                size: bytes.len() as u64,
                usage: usage,
                mapped_at_creation: false,
            });

            self.cache.spawn(
                UniformRenderData { buffer },
                Default::default(),
                Default::default(),
            );
        } else {
            index = self.pointer;

            let render_data = self.cache.buffer.get_mut_raw(index).unwrap();

            queue.write_buffer(&render_data.buffer, 0, bytes);
        }
        self.pointer += 1;

        index
    }
}

pub struct UniformCache {
    sets: HashMap<UniformSetKey, UniformSet>,
    device: RenderDevice,
    queue: RenderQueue,
}

impl UniformCache {
    pub fn new(device: &RenderDevice, queue: &RenderQueue) -> Self {
        Self {
            sets: Default::default(),
            device: device.clone(),
            queue: queue.clone(),
        }
    }
}

pub struct UniformIndex {
    pub key: UniformSetKey,
    pub index: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UniformSetKey {
    size: u64,
    usage: BufferUsages,
    name: String,
}

impl UniformCache {
    pub fn unset(&mut self) {
        for set in self.sets.values_mut() {
            set.unset();
        }
    }

    pub fn upload(&mut self, name: &str, bytes: &[u8], usage: BufferUsages) -> UniformIndex {
        let key: UniformSetKey = UniformSetKey {
            size: bytes.len() as u64,
            usage,
            name: name.to_string(),
        };
        let set = self.sets.entry(key.clone()).or_insert(Default::default());

        let index = set.get_or_create(&self.device, &self.queue, name, bytes, usage);

        UniformIndex { key, index }
    }
}
