use std::num::NonZero;

use encase::{
    ShaderType,
    private::{ArrayMetadata, BufferMut, Metadata, RuntimeSizedArray, WriteInto, Writer},
};
use nonmax::NonMaxU32;

use crate::{
    render_resource::{DynamicUniformBuffer, GpuArrayBufferIndex, GpuArrayBufferable},
    render_world::{RenderWorld, UniformIndex},
};
use draft_graphics::Limits;

const MAX_REASONABLE_UNIFORM_BUFFER_BINDING_SIZE: u64 = 1 << 20;

pub struct BatchedUniformBuffer<T: GpuArrayBufferable> {
    // Batches of fixed-size arrays of T are written to this buffer so that
    // each batch in a fixed-size array can be bound at a dynamic offset.
    uniforms: DynamicUniformBuffer<MaxCapacityArray<Vec<T>>>,
    // A batch of T are gathered into this `MaxCapacityArray` until it is full,
    // then it is written into the `DynamicUniformBuffer`, cleared, and new T
    // are gathered here, and so on for each batch.
    temp: MaxCapacityArray<Vec<T>>,
    current_offset: u32,
    dynamic_offset_alignment: u32,
}

impl<T: GpuArrayBufferable> BatchedUniformBuffer<T> {
    pub fn batch_size(limits: &Limits) -> usize {
        (limits
            .max_uniform_buffer_binding_size
            .min(MAX_REASONABLE_UNIFORM_BUFFER_BINDING_SIZE)
            / T::min_size().get()) as usize
    }

    pub fn new(limits: &Limits) -> Self {
        let capacity = Self::batch_size(limits);
        let alignment = limits.min_uniform_buffer_offset_alignment;

        Self {
            uniforms: DynamicUniformBuffer::new_with_alignment(alignment as u64),
            temp: MaxCapacityArray(Vec::with_capacity(capacity), capacity),
            current_offset: 0,
            dynamic_offset_alignment: alignment,
        }
    }

    pub fn push(&mut self, component: T) -> GpuArrayBufferIndex {
        let result = GpuArrayBufferIndex {
            index: self.temp.0.len() as u32,
            dynamic_offset: NonMaxU32::new(self.current_offset),
        };
        self.temp.0.push(component);
        if self.temp.0.len() == self.temp.1 {
            self.flush();
        }
        result
    }

    pub fn flush(&mut self) {
        self.uniforms.push(&self.temp);

        self.current_offset +=
            align_to_next(self.temp.size().get(), self.dynamic_offset_alignment as u64) as u32;

        self.temp.0.clear();
    }

    pub fn write(&mut self, name: &str, render_world: &mut RenderWorld) -> UniformIndex {
        if !self.temp.0.is_empty() {
            self.flush();
        }

        self.uniforms.write(name, render_world)
    }
}

#[inline]
fn align_to_next(value: u64, alignment: u64) -> u64 {
    debug_assert!(alignment.is_power_of_two());
    ((value - 1) | (alignment - 1)) + 1
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct MaxCapacityArray<T>(T, usize);

impl<T> ShaderType for MaxCapacityArray<T>
where
    T: ShaderType<ExtraMetadata = ArrayMetadata>,
{
    type ExtraMetadata = ArrayMetadata;

    const METADATA: Metadata<Self::ExtraMetadata> = T::METADATA;

    fn size(&self) -> NonZero<u64> {
        Self::METADATA.stride().mul(self.1.max(1) as u64).0
    }
}

impl<T> WriteInto for MaxCapacityArray<T>
where
    T: WriteInto + RuntimeSizedArray,
{
    fn write_into<B: BufferMut>(&self, writer: &mut Writer<B>) {
        debug_assert!(self.0.len() <= self.1);
        self.0.write_into(writer);
    }
}
