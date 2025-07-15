use fxhash::FxHashMap;

use crate::{
    FrameworkError, frame_graph::BufferInfo, gfx_base::RenderDevice, render_resource::RenderBuffer,
};

pub struct UniformBufferSet {
    desc: BufferInfo,
    buffers: Vec<RenderBuffer>,
    free: usize,
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

    fn get_or_create(&mut self, device: &RenderDevice) -> Result<RenderBuffer, FrameworkError> {
        if self.free < self.buffers.len() {
            let buffer = &self.buffers[self.free];
            self.free += 1;
            Ok(buffer.clone())
        } else {
            let buffer = device
                .wgpu_device()
                .create_buffer(&self.desc.get_buffer_desc());

            let key = format!("UniformBuffer_{}_{}", self.desc.size, self.buffers.len());

            self.buffers.push(RenderBuffer {
                key,
                value: buffer,
                desc: self.desc.clone(),
            });
            self.free = self.buffers.len();
            Ok(self.buffers.last().unwrap().clone())
        }
    }
}

pub struct UniformBufferCache {
    device: RenderDevice,
    cache: FxHashMap<BufferInfo, UniformBufferSet>,
}

impl UniformBufferCache {
    pub fn new(device: RenderDevice) -> Self {
        Self {
            device,
            cache: Default::default(),
        }
    }

    pub fn get_or_create(&mut self, desc: BufferInfo) -> Result<RenderBuffer, FrameworkError> {
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
