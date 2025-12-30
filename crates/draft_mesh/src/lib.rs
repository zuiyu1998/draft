mod primitives;
mod vertex;

pub use primitives::*;
pub use vertex::*;

use bytemuck::cast_slice;
use draft_graphics::{
    IndexFormat, PrimitiveTopology, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, uuid, visitor::*, warn};
use fyrox_resource::{Resource, ResourceData};
use std::{collections::BTreeMap, error::Error, path::Path};

pub type MeshResource = Resource<Mesh>;

#[derive(Debug, Clone, Reflect)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    pub fn index_format(&self) -> IndexFormat {
        match self {
            Indices::U16(_) => IndexFormat::Uint16,
            Indices::U32(_) => IndexFormat::Uint32,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Indices::U16(data) => data.len(),
            Indices::U32(data) => data.len(),
        }
    }
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U32(vec![])
    }
}

#[derive(Debug, Clone, Default, Reflect, TypeUuidProvider)]
#[type_uuid(id = "8a23a414-e66d-4e12-9628-92c6ab49c2f0")]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,
    #[reflect(hidden)]
    attributes: BTreeMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,
    #[reflect(hidden)]
    modifications_counter: u64,
}

impl Mesh {
    pub fn new(primitive_topology: PrimitiveTopology) -> Self {
        Mesh {
            primitive_topology,
            attributes: Default::default(),
            indices: None,
            modifications_counter: 0,
        }
    }

    pub fn modifications_counter(&self) -> u64 {
        self.modifications_counter
    }

    pub fn count_vertices(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        for (attribute_id, attribute_data) in &self.attributes {
            let attribute_len = attribute_data.values.len();
            if let Some(previous_vertex_count) = vertex_count {
                if previous_vertex_count != attribute_len {
                    let name = self
                        .attributes
                        .get(attribute_id)
                        .map(|data| data.attribute.name.to_string())
                        .unwrap_or_else(|| format!("{attribute_id:?}"));

                    warn!(
                        "{name} has a different vertex count ({attribute_len}) than other attributes ({previous_vertex_count}) in this mesh, \
                        all attributes will be truncated to match the smallest."
                    );
                    vertex_count = Some(core::cmp::min(previous_vertex_count, attribute_len));
                }
            } else {
                vertex_count = Some(attribute_len);
            }
        }

        vertex_count.unwrap_or(0)
    }

    pub fn get_vertex_size(&self) -> u64 {
        self.attributes
            .values()
            .map(|data| data.attribute.format.size())
            .sum()
    }

    pub fn get_vertex_buffer_size(&self) -> usize {
        let vertex_size = self.get_vertex_size() as usize;
        let vertex_count = self.count_vertices();
        vertex_count * vertex_size
    }

    pub fn count_indexs(&self) -> Option<usize> {
        self.indices.as_ref().map(|indices| match &indices {
            Indices::U16(indices) => indices.len(),
            Indices::U32(indices) => indices.len(),
        })
    }

    pub fn indices(&self) -> Option<&Indices> {
        self.indices.as_ref()
    }

    pub fn get_index_buffer_bytes(&self) -> Option<&[u8]> {
        self.indices.as_ref().map(|indices| match &indices {
            Indices::U16(indices) => cast_slice(&indices[..]),
            Indices::U32(indices) => cast_slice(&indices[..]),
        })
    }

    pub fn create_packed_vertex_buffer_data(&self) -> Vec<u8> {
        let mut attributes_interleaved_buffer = vec![0; self.get_vertex_buffer_size()];
        self.write_packed_vertex_buffer_data(&mut attributes_interleaved_buffer);
        attributes_interleaved_buffer
    }

    pub fn write_packed_vertex_buffer_data(&self, slice: &mut [u8]) {
        let vertex_size = self.get_vertex_size() as usize;
        let vertex_count = self.count_vertices();
        // bundle into interleaved buffers
        let mut attribute_offset = 0;
        for attribute_data in self.attributes.values() {
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

    pub fn get_mesh_vertex_buffer_layout(
        &self,
        mesh_vertex_buffer_layouts: &mut MeshVertexBufferLayouts,
    ) -> MeshVertexBufferLayoutRef {
        let mut attributes = Vec::with_capacity(self.attributes.len());
        let mut attribute_ids = Vec::with_capacity(self.attributes.len());
        let mut accumulated_offset = 0;
        for (index, data) in self.attributes.values().enumerate() {
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

    pub fn attribute_position() -> MeshhVertexAttribute {
        MeshhVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x3)
    }

    pub fn attribute_normal() -> MeshhVertexAttribute {
        MeshhVertexAttribute::new("Vertex_Normal", 1, VertexFormat::Float32x3)
    }

    pub fn attribute_uv_0() -> MeshhVertexAttribute {
        MeshhVertexAttribute::new("Vertex_Uv", 2, VertexFormat::Float32x2)
    }

    pub fn insert_indices(&mut self, indices: Indices) {
        self.indices = Some(indices);
    }

    pub fn with_inserted_indices(mut self, indices: Indices) -> Self {
        self.insert_indices(indices);
        self
    }

    pub fn insert_attribute(
        &mut self,
        attribute: MeshhVertexAttribute,
        values: impl Into<VertexAttributeValues>,
    ) {
        let values = values.into();
        let values_format = VertexFormat::from(&values);
        if values_format != attribute.format {
            panic!(
                "Failed to insert attribute. Invalid attribute format for {}. Given format is {values_format:?} but expected {:?}",
                attribute.name, attribute.format
            );
        }

        self.attributes
            .insert(attribute.id, MeshAttributeData { attribute, values });
    }

    pub fn with_inserted_attribute(
        mut self,
        attribute: MeshhVertexAttribute,
        values: impl Into<VertexAttributeValues>,
    ) -> Self {
        self.insert_attribute(attribute, values);
        self
    }
}

impl Visit for Mesh {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        // self.attributes.visit("Attributes", &mut region)?;

        Ok(())
    }
}

impl ResourceData for Mesh {
    fn type_uuid(&self) -> Uuid {
        <Mesh as fyrox_core::TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, _path: &Path) -> Result<(), Box<dyn Error>> {
        // TODO: Add saving.
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
