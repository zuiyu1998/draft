use crate::VertexFormat;
use fyrox_core::{reflect::*, visitor::*};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Reflect, Visit, Default)]
pub struct VertexAttributeId(u64);

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct VertexAttributeDescriptor {
    pub name: String,
    pub id: VertexAttributeId,
    pub format: VertexFormat,
}

#[derive(Clone, Debug, Reflect, Visit)]
pub enum VertexAttributeValues {
    Float32(Vec<f32>),
    Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(Vec<[u16; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Snorm8x2(Vec<[i8; 2]>),
    Uint8x2(Vec<[u8; 2]>),
    Unorm8x2(Vec<[u8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint8x4(Vec<[u8; 4]>),
    Unorm8x4(Vec<[u8; 4]>),
}

impl Default for VertexAttributeValues {
    fn default() -> Self {
        VertexAttributeValues::Uint32(vec![])
    }
}

impl From<&VertexAttributeValues> for VertexFormat {
    fn from(values: &VertexAttributeValues) -> Self {
        match values {
            VertexAttributeValues::Float32(_) => VertexFormat::Float32,
            VertexAttributeValues::Sint32(_) => VertexFormat::Sint32,
            VertexAttributeValues::Uint32(_) => VertexFormat::Uint32,
            VertexAttributeValues::Float32x2(_) => VertexFormat::Float32x2,
            VertexAttributeValues::Sint32x2(_) => VertexFormat::Sint32x2,
            VertexAttributeValues::Uint32x2(_) => VertexFormat::Uint32x2,
            VertexAttributeValues::Float32x3(_) => VertexFormat::Float32x3,
            VertexAttributeValues::Sint32x3(_) => VertexFormat::Sint32x3,
            VertexAttributeValues::Uint32x3(_) => VertexFormat::Uint32x3,
            VertexAttributeValues::Float32x4(_) => VertexFormat::Float32x4,
            VertexAttributeValues::Sint32x4(_) => VertexFormat::Sint32x4,
            VertexAttributeValues::Uint32x4(_) => VertexFormat::Uint32x4,
            VertexAttributeValues::Sint16x2(_) => VertexFormat::Sint16x2,
            VertexAttributeValues::Snorm16x2(_) => VertexFormat::Snorm16x2,
            VertexAttributeValues::Uint16x2(_) => VertexFormat::Uint16x2,
            VertexAttributeValues::Unorm16x2(_) => VertexFormat::Unorm16x2,
            VertexAttributeValues::Sint16x4(_) => VertexFormat::Sint16x4,
            VertexAttributeValues::Snorm16x4(_) => VertexFormat::Snorm16x4,
            VertexAttributeValues::Uint16x4(_) => VertexFormat::Uint16x4,
            VertexAttributeValues::Unorm16x4(_) => VertexFormat::Unorm16x4,
            VertexAttributeValues::Sint8x2(_) => VertexFormat::Sint8x2,
            VertexAttributeValues::Snorm8x2(_) => VertexFormat::Snorm8x2,
            VertexAttributeValues::Uint8x2(_) => VertexFormat::Uint8x2,
            VertexAttributeValues::Unorm8x2(_) => VertexFormat::Unorm8x2,
            VertexAttributeValues::Sint8x4(_) => VertexFormat::Sint8x4,
            VertexAttributeValues::Snorm8x4(_) => VertexFormat::Snorm8x4,
            VertexAttributeValues::Uint8x4(_) => VertexFormat::Uint8x4,
            VertexAttributeValues::Unorm8x4(_) => VertexFormat::Unorm8x4,
        }
    }
}

impl VertexAttributeValues {
    pub fn len(&self) -> usize {
        match self {
            VertexAttributeValues::Float32(values) => values.len(),
            VertexAttributeValues::Sint32(values) => values.len(),
            VertexAttributeValues::Uint32(values) => values.len(),
            VertexAttributeValues::Float32x2(values) => values.len(),
            VertexAttributeValues::Sint32x2(values) => values.len(),
            VertexAttributeValues::Uint32x2(values) => values.len(),
            VertexAttributeValues::Float32x3(values) => values.len(),
            VertexAttributeValues::Sint32x3(values) => values.len(),
            VertexAttributeValues::Uint32x3(values) => values.len(),
            VertexAttributeValues::Float32x4(values) => values.len(),
            VertexAttributeValues::Sint32x4(values) => values.len(),
            VertexAttributeValues::Uint32x4(values) => values.len(),
            VertexAttributeValues::Sint16x2(values) => values.len(),
            VertexAttributeValues::Snorm16x2(values) => values.len(),
            VertexAttributeValues::Uint16x2(values) => values.len(),
            VertexAttributeValues::Unorm16x2(values) => values.len(),
            VertexAttributeValues::Sint16x4(values) => values.len(),
            VertexAttributeValues::Snorm16x4(values) => values.len(),
            VertexAttributeValues::Uint16x4(values) => values.len(),
            VertexAttributeValues::Unorm16x4(values) => values.len(),
            VertexAttributeValues::Sint8x2(values) => values.len(),
            VertexAttributeValues::Snorm8x2(values) => values.len(),
            VertexAttributeValues::Uint8x2(values) => values.len(),
            VertexAttributeValues::Unorm8x2(values) => values.len(),
            VertexAttributeValues::Sint8x4(values) => values.len(),
            VertexAttributeValues::Snorm8x4(values) => values.len(),
            VertexAttributeValues::Uint8x4(values) => values.len(),
            VertexAttributeValues::Unorm8x4(values) => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct VertexAttributeData {
    pub desc: VertexAttributeDescriptor,
    pub values: VertexAttributeValues,
}

#[derive(Reflect, Clone, Visit, Default, Debug)]
pub struct VertexBuffer {
    attributes: BTreeMap<VertexAttributeId, VertexAttributeData>,
    #[visit(optional)]
    modifications_counter: u64,
    #[visit(optional)]
    layout_hash: u64,
}

impl VertexBuffer {
    pub fn insert_attribute(
        &mut self,
        desc: VertexAttributeDescriptor,
        values: impl Into<VertexAttributeValues>,
    ) {
        let values = values.into();
        let values_format = VertexFormat::from(&values);
        if values_format != desc.format {
            panic!(
                "Failed to insert attribute. Invalid attribute format for {}. Given format is {values_format:?} but expected {:?}",
                desc.name, desc.format
            );
        }

        self.attributes
            .insert(desc.id, VertexAttributeData { desc, values });
    }
}
