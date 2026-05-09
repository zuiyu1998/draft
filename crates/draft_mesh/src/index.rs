use fyrox_core::reflect::*;

#[derive(Debug, Clone, Default, Reflect)]
pub struct IndexBuffer {
    indices: Indices,
    #[reflect(hidden)]
    pub modifications_counter: u64,
}

impl IndexBuffer {
    pub fn get_mut<'a>(&'a mut self) -> IndexBufferMut<'a> {
        IndexBufferMut { index_buffer: self }
    }
}

pub struct IndexBufferMut<'a> {
    index_buffer: &'a mut IndexBuffer,
}

impl<'a> IndexBufferMut<'a> {
    pub fn set_indices_with_u16(&mut self, indices: &[u16]) {
        self.index_buffer.indices = Indices::U16(indices.to_vec());
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U16(vec![])
    }
}
