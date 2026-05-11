use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    sync::Arc,
};

use bytemuck::cast_slice;
use draft_graphics::{BufferAddress, VertexAttribute, VertexFormat, VertexStepMode};
use fyrox_core::reflect::*;

#[derive(Default, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexBufferLayout {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MeshVertexBufferLayout {
    pub(crate) attribute_ids: Vec<MeshVertexAttributeId>,
    pub(crate) layout: VertexBufferLayout,
}

#[derive(Clone, Debug)]
pub struct MeshVertexBufferLayoutRef(pub Arc<MeshVertexBufferLayout>);

impl PartialEq for MeshVertexBufferLayoutRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for MeshVertexBufferLayoutRef {}

impl Hash for MeshVertexBufferLayoutRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the address of the underlying data, so two layouts that share the same
        // `MeshVertexBufferLayout` will have the same hash.
        (Arc::as_ptr(&self.0) as usize).hash(state);
    }
}

#[derive(Clone, Default)]
pub struct MeshVertexBufferLayouts(HashSet<Arc<MeshVertexBufferLayout>>);

impl MeshVertexBufferLayouts {
    pub fn insert(&mut self, layout: MeshVertexBufferLayout) -> MeshVertexBufferLayoutRef {
        let layout = Arc::new(layout);

        if !self.0.contains(&layout) {
            self.0.insert(layout.clone());
        }

        MeshVertexBufferLayoutRef(layout)
    }
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct VertexBuffer {
    attributes: HashMap<MeshVertexAttributeId, MeshAttributeData>,
    attribute_ids: Vec<MeshVertexAttributeId>,

    #[reflect(hidden)]
    pub modifications_counter: u64,
}

impl VertexBuffer {
    pub fn get_vertex_size(&self) -> u64 {
        self.attributes
            .values()
            .map(|data| data.attribute.format.size())
            .sum()
    }

    pub fn count_vertices(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        let mesh_attributes = &self.attributes;

        for (_attribute_id, attribute_data) in mesh_attributes {
            let attribute_len = attribute_data.values.len();
            if let Some(previous_vertex_count) = vertex_count {
                if previous_vertex_count != attribute_len {
                    vertex_count = Some(core::cmp::min(previous_vertex_count, attribute_len));
                }
            } else {
                vertex_count = Some(attribute_len);
            }
        }

        vertex_count.unwrap_or(0)
    }

    pub fn get_vertex_buffer_size(&self) -> usize {
        let vertex_size = self.get_vertex_size() as usize;
        let vertex_count = self.count_vertices();
        vertex_count * vertex_size
    }

    pub fn write_packed_vertex_buffer_data(&self, slice: &mut [u8]) {
        let mesh_attributes = &self.attributes;

        let vertex_size = self.get_vertex_size() as usize;
        let vertex_count = self.count_vertices();
        // bundle into interleaved buffers
        let mut attribute_offset = 0;
        for attribute_data in mesh_attributes.values() {
            let attribute_size = attribute_data.attribute.format.size() as usize;
            let attributes_bytes = attribute_data.values.get_bytes();
            for (vertex_index, attribute_bytes) in attributes_bytes
                .chunks_exact(attribute_size)
                .take(vertex_count)
                .enumerate()
            {
                let offset = vertex_index * vertex_size + attribute_offset;
                slice[offset..offset + attribute_size].copy_from_slice(attribute_bytes);
            }

            attribute_offset += attribute_size;
        }
    }

    pub fn create_packed_vertex_buffer_data(&self) -> Vec<u8> {
        let mut attributes_interleaved_buffer = vec![0; self.get_vertex_buffer_size()];
        self.write_packed_vertex_buffer_data(&mut attributes_interleaved_buffer);
        attributes_interleaved_buffer
    }

    pub fn get_mut<'a>(&'a mut self) -> VertexBufferMut<'a> {
        VertexBufferMut {
            vertex_buffer: self,
        }
    }

    fn insert_attribute(&mut self, attribute: MeshVertexAttribute, values: VertexAttributeValues) {
        self.insert_attribute_id(&attribute.id);
        self.attributes
            .insert(attribute.id, MeshAttributeData { attribute, values });
    }

    fn insert_attribute_id(&mut self, attribute_id: &MeshVertexAttributeId) {
        if !self.attribute_ids.contains(attribute_id) {
            self.attribute_ids.push(*attribute_id);
            self.attribute_ids.sort();
        }
    }

    pub fn get_mesh_vertex_buffer_layout(
        &self,
        mesh_vertex_buffer_layouts: &mut MeshVertexBufferLayouts,
    ) -> MeshVertexBufferLayoutRef {
        let mesh_attributes = &self.attributes;

        let mut attributes = Vec::with_capacity(mesh_attributes.len());
        let mut attribute_ids = Vec::with_capacity(mesh_attributes.len());
        let mut accumulated_offset = 0;
        for (index, attribute_id) in self.attribute_ids.iter().enumerate() {
            let data = mesh_attributes.get(attribute_id).unwrap();

            attribute_ids.push(data.attribute.id);
            attributes.push(VertexAttribute {
                offset: accumulated_offset,
                format: data.attribute.format,
                shader_location: index as u32,
            });
            accumulated_offset += data.attribute.format.size();
        }

        let layout = MeshVertexBufferLayout {
            layout: VertexBufferLayout {
                array_stride: accumulated_offset,
                step_mode: VertexStepMode::Vertex,
                attributes,
            },
            attribute_ids,
        };
        mesh_vertex_buffer_layouts.insert(layout)
    }
}

pub struct VertexBufferMut<'a> {
    vertex_buffer: &'a mut VertexBuffer,
}

impl<'a> Drop for VertexBufferMut<'a> {
    fn drop(&mut self) {
        self.vertex_buffer.modifications_counter += 1;
    }
}

impl<'a> VertexBufferMut<'a> {
    pub fn insert_attribute(
        &mut self,
        attribute: MeshVertexAttribute,
        values: impl Into<VertexAttributeValues>,
    ) {
        let values = values.into();
        let values_format = VertexFormat::from(&values);
        if values_format != attribute.format {
            panic!(
                "Failed to insert attribute. Given format is {values_format:?} but expected {:?}",
                attribute.format
            );
        }

        self.vertex_buffer.insert_attribute(attribute, values);
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct MeshAttributeData {
    pub(crate) attribute: MeshVertexAttribute,
    pub(crate) values: VertexAttributeValues,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
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

impl VertexAttributeValues {
    /// Returns the number of vertices in this [`VertexAttributeValues`]. For a single
    /// mesh, all of the [`VertexAttributeValues`] must have the same length.
    #[expect(
        clippy::match_same_arms,
        reason = "Although the `values` binding on some match arms may have matching types, each variant has different semantics; thus it's not guaranteed that they will use the same type forever."
    )]
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

    /// Returns `true` if there are no vertices in this [`VertexAttributeValues`].
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the values as float triples if possible.
    pub fn as_float3(&self) -> Option<&[[f32; 3]]> {
        match self {
            VertexAttributeValues::Float32x3(values) => Some(values),
            _ => None,
        }
    }

    // TODO: add vertex format as parameter here and perform type conversions
    /// Flattens the [`VertexAttributeValues`] into a sequence of bytes. This is
    /// useful for serialization and sending to the GPU.
    #[expect(
        clippy::match_same_arms,
        reason = "Although the `values` binding on some match arms may have matching types, each variant has different semantics; thus it's not guaranteed that they will use the same type forever."
    )]
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            VertexAttributeValues::Float32(values) => cast_slice(values),
            VertexAttributeValues::Sint32(values) => cast_slice(values),
            VertexAttributeValues::Uint32(values) => cast_slice(values),
            VertexAttributeValues::Float32x2(values) => cast_slice(values),
            VertexAttributeValues::Sint32x2(values) => cast_slice(values),
            VertexAttributeValues::Uint32x2(values) => cast_slice(values),
            VertexAttributeValues::Float32x3(values) => cast_slice(values),
            VertexAttributeValues::Sint32x3(values) => cast_slice(values),
            VertexAttributeValues::Uint32x3(values) => cast_slice(values),
            VertexAttributeValues::Float32x4(values) => cast_slice(values),
            VertexAttributeValues::Sint32x4(values) => cast_slice(values),
            VertexAttributeValues::Uint32x4(values) => cast_slice(values),
            VertexAttributeValues::Sint16x2(values) => cast_slice(values),
            VertexAttributeValues::Snorm16x2(values) => cast_slice(values),
            VertexAttributeValues::Uint16x2(values) => cast_slice(values),
            VertexAttributeValues::Unorm16x2(values) => cast_slice(values),
            VertexAttributeValues::Sint16x4(values) => cast_slice(values),
            VertexAttributeValues::Snorm16x4(values) => cast_slice(values),
            VertexAttributeValues::Uint16x4(values) => cast_slice(values),
            VertexAttributeValues::Unorm16x4(values) => cast_slice(values),
            VertexAttributeValues::Sint8x2(values) => cast_slice(values),
            VertexAttributeValues::Snorm8x2(values) => cast_slice(values),
            VertexAttributeValues::Uint8x2(values) => cast_slice(values),
            VertexAttributeValues::Unorm8x2(values) => cast_slice(values),
            VertexAttributeValues::Sint8x4(values) => cast_slice(values),
            VertexAttributeValues::Snorm8x4(values) => cast_slice(values),
            VertexAttributeValues::Uint8x4(values) => cast_slice(values),
            VertexAttributeValues::Unorm8x4(values) => cast_slice(values),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct MeshVertexAttribute {
    /// The _unique_ id of the vertex attribute. This will also determine sort ordering
    /// when generating vertex buffers. Built-in / standard attributes will use "close to zero"
    /// indices. When in doubt, use a random / very large u64 to avoid conflicts.
    pub id: MeshVertexAttributeId,

    /// The format of the vertex attribute.
    pub format: VertexFormat,
}

#[derive(Debug, Clone, PartialEq, Reflect, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct MeshVertexAttributeId(u64);
