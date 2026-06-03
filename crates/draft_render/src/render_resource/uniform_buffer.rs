use draft_graphics::BufferUsages;
use encase::{
    DynamicUniformBuffer as DynamicUniformBufferWrapper, ShaderType, internal::WriteInto,
};
use std::marker::PhantomData;

use crate::render_world::{RenderWorld, UniformIndex};

pub struct DynamicUniformBuffer<T: ShaderType> {
    scratch: DynamicUniformBufferWrapper<Vec<u8>>,
    buffer_usage: BufferUsages,
    _marker: PhantomData<fn() -> T>,
}

impl<T: ShaderType + WriteInto> DynamicUniformBuffer<T> {
    pub fn new_with_alignment(alignment: u64) -> Self {
        Self {
            scratch: DynamicUniformBufferWrapper::new_with_alignment(Vec::new(), alignment),
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn push(&mut self, value: &T) -> u32 {
        self.scratch.write(value).unwrap() as u32
    }

    pub fn write(&mut self, name: &str, render_world: &mut RenderWorld) -> UniformIndex {
        render_world.upload_uniform(name, self.scratch.as_ref(), self.buffer_usage)
    }
}

impl<T: ShaderType> Default for DynamicUniformBuffer<T> {
    fn default() -> Self {
        Self {
            scratch: DynamicUniformBufferWrapper::new(Vec::new()),
            buffer_usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            _marker: PhantomData,
        }
    }
}
