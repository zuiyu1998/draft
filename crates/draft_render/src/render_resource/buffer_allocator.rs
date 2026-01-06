use std::collections::HashMap;

use draft_graphics::gfx_base::{BufferDescriptor, GpuBuffer, RenderDevice};

pub struct BufferAllocator {
    render_device: RenderDevice,
    data: HashMap<BufferDescriptor, BufferSet>,
}

impl BufferAllocator {
    pub fn new(render_device: RenderDevice) -> Self {
        Self {
            render_device,
            data: Default::default(),
        }
    }

    pub fn unset(&mut self) {
        for set in self.data.values_mut() {
            set.unset();
        }
    }

    pub fn get_buffer(&self, handle: &BufferHandle) -> GpuBuffer {
        self.data
            .get(&handle.desc)
            .map(|set| set.buffers[handle.index].clone())
            .expect("buffer must have")
    }

    pub fn allocate(&mut self, desc: &BufferDescriptor) -> BufferHandle {
        match self.data.get_mut(&desc) {
            None => {
                let mut buffer_set = BufferSet::default();
                let handle = buffer_set.allocate(&self.render_device, desc.clone());

                self.data.insert(desc.clone(), buffer_set);

                handle
            }
            Some(buffer_set) => buffer_set.allocate(&self.render_device, desc.clone()),
        }
    }
}

pub struct BufferHandle {
    pub desc: BufferDescriptor,
    pub index: usize,
}

#[derive(Default)]
pub struct BufferSet {
    buffers: Vec<GpuBuffer>,
    free: usize,
}

impl BufferSet {
    pub fn unset(&mut self) {
        self.free = 0;
    }

    pub fn allocate(
        &mut self,
        render_device: &RenderDevice,
        desc: BufferDescriptor,
    ) -> BufferHandle {
        let index = self.free;

        if self.free < self.buffers.len() {
            self.free += 1;
            BufferHandle { desc, index }
        } else {
            self.buffers.push(render_device.create_buffer(&desc));

            BufferHandle { desc, index }
        }
    }
}
