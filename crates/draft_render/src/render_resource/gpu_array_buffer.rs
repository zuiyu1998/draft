use std::marker::PhantomData;

use draft_graphics::gfx_base::{RenderDevice, RenderQueue};
use draft_material::{BindGroupLayoutEntryBuilder, binding_types::storage_buffer_read_only};
use encase::{ShaderSize, ShaderType, private::WriteInto};
use nonmax::NonMaxU32;

use crate::{BufferAllocator, BufferVec};

pub trait GpuArrayBufferable: ShaderType + ShaderSize + WriteInto + Clone {}

impl<T: ShaderType + ShaderSize + WriteInto + Clone> GpuArrayBufferable for T {}

#[derive(Clone)]
pub struct GpuArrayBufferIndex<T: GpuArrayBufferable> {
    /// The index to use in a shader into the array.
    pub index: u32,
    /// The dynamic offset to use when setting the bind group in a pass.
    /// Only used on platforms that don't support storage buffers.
    pub dynamic_offset: Option<NonMaxU32>,
    pub element_type: PhantomData<T>,
}

pub enum GpuArrayBuffer<T: GpuArrayBufferable> {
    Storage(BufferVec<T>),
}

impl<T: GpuArrayBufferable> GpuArrayBuffer<T> {
    pub fn binding_layout(_device: &RenderDevice) -> BindGroupLayoutEntryBuilder {
        storage_buffer_read_only::<T>(false)
    }

    pub fn write_buffer(&mut self, buffer_allocator: &mut BufferAllocator, queue: &RenderQueue) {
        match self {
            GpuArrayBuffer::Storage(buffer) => buffer.write_buffer(buffer_allocator, queue),
        }
    }
}
