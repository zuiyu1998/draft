use draft_graphics::{
    BufferUsages,
    gfx_base::{BufferDescriptor, RenderQueue},
};
use std::iter;

use crate::{Buffer, BufferAllocator};

pub struct BufferVec {
    data: Vec<u8>,
    buffer: Option<Buffer>,
    capacity: usize,
    buffer_usage: BufferUsages,
    label: String,
    label_changed: bool,
}

impl BufferVec {
    pub fn new(buffer_usage: BufferUsages, label: &str) -> Self {
        Self {
            data: vec![],
            buffer: None,
            capacity: 0,
            buffer_usage,
            label: label.to_string(),
            label_changed: false,
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> usize {
        let element_size = bytes.len() as usize;
        let offset = self.data.len();

        // TODO: Consider using unsafe code to push uninitialized, to prevent
        // the zeroing. It shows up in profiles.
        self.data.extend(iter::repeat_n(0, element_size));

        // Take a slice of the new data for `write_into` to use. This is
        // important: it hoists the bounds check up here so that the compiler
        // can eliminate all the bounds checks that `write_into` will emit.
        let dest = &mut self.data[offset..(offset + element_size)];

        dest.copy_from_slice(bytes);

        offset / element_size as usize
    }

    pub fn reserve(&mut self, capacity: usize, buffer_allocator: &mut BufferAllocator) {
        if capacity <= self.capacity && !self.label_changed {
            return;
        }

        self.capacity = capacity;

        let desc = BufferDescriptor {
            label: Some(self.label.clone()),
            size: capacity as u64,
            usage: BufferUsages::COPY_DST | self.buffer_usage,
            mapped_at_creation: false,
        };

        let handle = buffer_allocator.allocate(&desc);
        let buffer = buffer_allocator.get_buffer(&handle);

        self.buffer = Some(Buffer::new(&self.label, buffer, desc));
    }

    pub fn write_buffer(&mut self, buffer_allocator: &mut BufferAllocator, queue: &RenderQueue) {
        if self.data.is_empty() {
            return;
        }

        self.reserve(self.data.len(), buffer_allocator);

        let Some(buffer) = &self.buffer else { return };
        queue.write_buffer(buffer.value(), 0, &self.data);
    }
}
