use bytes::BytesMut;
use fxhash::FxHashMap;

use crate::{ResourceKey, Std140};

#[derive(Default)]
pub struct BufferAllocator {
    buffers: FxHashMap<ResourceKey, Buffer>,
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
    fn get_or_create(&mut self, key: ResourceKey) -> &mut Buffer {
        self.buffers.entry(key).or_default()
    }

    pub fn write<T: Std140>(&mut self, key: &ResourceKey, value: T) -> u32 {
        let buffer = self.get_or_create(key.clone());

        let mut offset = buffer.last_offset;

        value.write(&mut buffer.bytes, &mut offset);

        buffer.last_offset = offset;

        offset
    }

    pub fn clear(&mut self) {
        for buffer in self.buffers.values_mut() {
            buffer.clear();
        }
    }
}
