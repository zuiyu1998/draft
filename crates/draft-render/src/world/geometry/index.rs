use crate::gfx_base::RawIndexFormat;
use bytemuck::cast_slice;
use fyrox_core::visitor::Visit;
use fyrox_core::{reflect::*, visitor::*};

#[derive(Reflect, Clone, Visit, Debug)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    pub fn index_format(&self) -> RawIndexFormat {
        match self {
            Indices::U16(_) => RawIndexFormat::Uint16,
            Indices::U32(_) => RawIndexFormat::Uint32,
        }
    }

    pub fn create_buffer(&self) -> Vec<u8> {
        let bytes: &[u8] = match &self {
            Indices::U16(indices) => cast_slice(&indices[..]),
            Indices::U32(indices) => cast_slice(&indices[..]),
        };
        bytes.to_vec()
    }

    pub fn len(&self) -> usize {
        match self {
            Indices::U16(vec) => vec.len(),
            Indices::U32(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Indices::U16(vec) => vec.is_empty(),
            Indices::U32(vec) => vec.is_empty(),
        }
    }
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U32(vec![])
    }
}

#[derive(Reflect, Clone, Visit, Default, Debug)]
pub struct Index {
    pub indices: Option<Indices>,
    #[visit(optional)]
    modifications_counter: u64,
}

impl From<Vec<u32>> for Index {
    fn from(value: Vec<u32>) -> Self {
        Index {
            indices: Some(Indices::U32(value)),
            modifications_counter: 0,
        }
    }
}

impl From<Vec<u16>> for Index {
    fn from(value: Vec<u16>) -> Self {
        Index {
            indices: Some(Indices::U16(value)),
            modifications_counter: 0,
        }
    }
}
