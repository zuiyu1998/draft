use bytes::BytesMut;
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;
use image::EncodableLayout;
use wgpu::BufferUsages;

use crate::{Std140, frame_graph::BufferInfo};

#[derive(Default)]
pub struct BufferAllocator {
    buffers: FxHashMap<ImmutableString, Buffer>,
}

#[derive(Default)]
pub struct Buffer {
    bytes: BytesMut,
    last_offset: u32,
}

impl Buffer {
    pub fn clear(&mut self) {
        self.bytes.clear();
        self.last_offset = 0;
    }
}

impl BufferAllocator {
    pub fn get_bytes(&self, key: &ImmutableString) -> &[u8] {
        self.buffers.get(key).unwrap().bytes.as_bytes()
    }

    pub fn get_buffer_info(&self, key: &ImmutableString, usage: BufferUsages) -> BufferInfo {
        let size = self.buffers.get(key).unwrap().bytes.len();

        BufferInfo {
            label: None,
            size: size as u64,
            usage,
            mapped_at_creation: false,
        }
    }

    fn get_or_create(&mut self, key: ImmutableString) -> &mut Buffer {
        self.buffers.entry(key).or_default()
    }

    pub fn write<T: Std140>(&mut self, key: &ImmutableString, value: T) -> (u32, u32) {
        let buffer = self.get_or_create(key.clone());

        let mut offset = buffer.last_offset;

        value.write(&mut buffer.bytes, &mut offset);

        let size = offset - buffer.last_offset;

        buffer.last_offset = offset;

        (offset, size)
    }

    pub fn clear(&mut self) {
        for buffer in self.buffers.values_mut() {
            buffer.clear();
        }
    }
}
