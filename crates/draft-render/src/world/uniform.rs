use fxhash::FxHashMap;

use crate::{
    frame_graph::BufferInfo,
    gfx_base::{RenderDevice, RenderQueue},
    render_resource::RenderBuffer,
};

pub struct UniformBufferKey {
    desc: BufferInfo,
    index: usize,
}

pub struct UniformBufferSet {
    desc: BufferInfo,
    buffers: Vec<RenderBuffer>,
    free: usize,
}

fn get_buffer_key(desc: &BufferInfo, index: usize) -> String {
    match &desc.label {
        Some(label) => format!("UniformBuffer_{}_{}_{}", label, desc.size, index),
        None => format!("UniformBuffer_{}_{}", desc.size, index),
    }
}

impl UniformBufferSet {
    pub fn new(desc: BufferInfo) -> Self {
        UniformBufferSet {
            desc,
            buffers: vec![],
            free: 0,
        }
    }

    fn mark_unused(&mut self) {
        self.free = 0;
    }

    fn get_or_create(&mut self, device: &RenderDevice) -> UniformBufferKey {
        let index = if self.free < self.buffers.len() {
            let index = self.free;
            self.free += 1;
            index
        } else {
            let buffer = device
                .wgpu_device()
                .create_buffer(&self.desc.get_buffer_desc());

            let key = get_buffer_key(&self.desc, self.buffers.len());
            let index = self.buffers.len();
            self.buffers.push(RenderBuffer {
                key,
                value: buffer,
                desc: self.desc.clone(),
            });

            self.free = self.buffers.len();
            index
        };

        UniformBufferKey {
            desc: self.desc.clone(),
            index,
        }
    }
}

pub struct UniformBufferCache {
    device: RenderDevice,
    queue: RenderQueue,
    cache: FxHashMap<BufferInfo, UniformBufferSet>,
}

impl UniformBufferCache {
    pub fn new(device: RenderDevice, queue: RenderQueue) -> Self {
        Self {
            device,
            cache: Default::default(),
            queue,
        }
    }

    pub fn get_render_buffer(&self, key: &UniformBufferKey) -> &RenderBuffer {
        self.try_get_render_buffer(key)
            .expect("must have uniform buffer")
    }

    pub fn try_get_render_buffer(&self, key: &UniformBufferKey) -> Option<&RenderBuffer> {
        self.cache
            .get(&key.desc)
            .and_then(|set| set.buffers.get(key.index))
    }

    pub fn upload_bytes(&mut self, key: &UniformBufferKey, bytes: &[u8]) {
        if let Some(set) = self.cache.get_mut(&key.desc) {
            if let Some(render_buffer) = set.buffers.get_mut(key.index) {
                self.queue
                    .wgpu_queue()
                    .write_buffer(&render_buffer.value, 0, bytes);
            }
        }
    }

    pub fn get_or_create(&mut self, desc: BufferInfo) -> UniformBufferKey {
        let set = self
            .cache
            .entry(desc.clone())
            .or_insert(UniformBufferSet::new(desc));
        set.get_or_create(&self.device)
    }

    pub fn mark_all_unused(&mut self) {
        for set in self.cache.values_mut() {
            set.mark_unused();
        }
    }
}
