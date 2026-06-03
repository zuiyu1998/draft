use crate::{
    render_resource::{BatchedUniformBuffer, GpuArrayBufferable},
    render_world::{RenderWorld, UniformIndex},
};
use draft_graphics::Limits;
use nonmax::NonMaxU32;

pub enum GpuArrayBuffer<T: GpuArrayBufferable> {
    Uniform(BatchedUniformBuffer<T>),
}

impl<T: GpuArrayBufferable> GpuArrayBuffer<T> {
    pub fn new(limits: &Limits) -> Self {
        GpuArrayBuffer::Uniform(BatchedUniformBuffer::new(limits))
    }

    pub fn push(&mut self, value: T) -> GpuArrayBufferIndex {
        match self {
            GpuArrayBuffer::Uniform(buffer) => buffer.push(value),
        }
    }

    pub fn write(&mut self, name: &str, render_world: &mut RenderWorld) -> UniformIndex {
        match self {
            GpuArrayBuffer::Uniform(buffer) => buffer.write(name, render_world),
        }
    }
}

pub struct GpuArrayBufferIndex {
    /// The index to use in a shader into the array.
    pub index: u32,
    /// The dynamic offset to use when setting the bind group in a pass.
    /// Only used on platforms that don't support storage buffers.
    pub dynamic_offset: Option<NonMaxU32>,
}
