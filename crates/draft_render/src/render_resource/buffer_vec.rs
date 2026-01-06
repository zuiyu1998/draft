use draft_graphics::{
    BufferUsages,
    gfx_base::{BufferDescriptor, RenderQueue},
};
use encase::{
    ShaderType,
    internal::{WriteInto, Writer},
};
use std::{iter, marker::PhantomData};

use crate::{Buffer, BufferAllocator};

pub struct BufferVec<T>
where
    T: ShaderType + WriteInto,
{
    data: Vec<u8>,
    buffer: Option<Buffer>,
    capacity: usize,
    buffer_usage: BufferUsages,
    label: String,
    label_changed: bool,
    phantom: PhantomData<T>,
}

impl<T> BufferVec<T>
where
    T: ShaderType + WriteInto,
{
    pub fn new(buffer_usage: BufferUsages, label: &str) -> Self {
        Self {
            data: vec![],
            buffer: None,
            capacity: 0,
            buffer_usage,
            label: label.to_string(),
            label_changed: false,
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        let element_size = u64::from(T::min_size()) as usize;
        let offset = self.data.len();

        // TODO: Consider using unsafe code to push uninitialized, to prevent
        // the zeroing. It shows up in profiles.
        self.data.extend(iter::repeat_n(0, element_size));

        // Take a slice of the new data for `write_into` to use. This is
        // important: it hoists the bounds check up here so that the compiler
        // can eliminate all the bounds checks that `write_into` will emit.
        let mut dest = &mut self.data[offset..(offset + element_size)];
        value.write_into(&mut Writer::new(&value, &mut dest, 0).unwrap());

        offset / u64::from(T::min_size()) as usize
    }

    pub fn reserve(&mut self, capacity: usize, buffer_allocator: &mut BufferAllocator) {
        if capacity <= self.capacity && !self.label_changed {
            return;
        }

        self.capacity = capacity;
        let size = u64::from(T::min_size()) as usize * capacity;

        let desc = BufferDescriptor {
            label: Some(self.label.clone()),
            size: size as u64,
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

        self.reserve(
            self.data.len() / u64::from(T::min_size()) as usize,
            buffer_allocator,
        );

        let Some(buffer) = &self.buffer else { return };
        queue.write_buffer(buffer.value(), 0, &self.data);
    }
}
