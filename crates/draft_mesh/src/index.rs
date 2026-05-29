use bytemuck::cast_slice;
use draft_graphics::IndexFormat;
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

    pub fn create_packed_index_buffer_data(&self) -> Vec<u8> {
        match &self.indices {
            Indices::U16(indices) => cast_slice(indices).to_vec(),
            Indices::U32(indices) => cast_slice(indices).to_vec(),
        }
    }

    pub fn get_index_format(&self) -> IndexFormat {
        match &self.indices {
            Indices::U16(_) => IndexFormat::Uint16,
            Indices::U32(_) => IndexFormat::Uint32,
        }
    }

    pub fn len(&self) -> usize {
        match &self.indices {
            Indices::U16(buffer) => buffer.len(),
            Indices::U32(buffer) => buffer.len(),
        }
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
