use draft_graphics::gfx_base::RenderQueue;
use nonmax::NonMaxU32;

use crate::{BufferAllocator, BufferVec};

#[derive(Clone)]
pub struct GpuArrayBufferIndex {
    /// The index to use in a shader into the array.
    pub index: u32,
    /// The dynamic offset to use when setting the bind group in a pass.
    /// Only used on platforms that don't support storage buffers.
    pub dynamic_offset: Option<NonMaxU32>,
}

pub enum GpuArrayBuffer {
    Storage(BufferVec),
}

impl GpuArrayBuffer {
    pub fn write_buffer(&mut self, buffer_allocator: &mut BufferAllocator, queue: &RenderQueue) {
        match self {
            GpuArrayBuffer::Storage(buffer) => buffer.write_buffer(buffer_allocator, queue),
        }
    }
}
