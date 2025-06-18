use crate::{RawVertexFormat, VertexAttribute, VertexBuffer, VertexBufferLayout, VertexStepMode};
use fyrox_core::{reflect::*, visitor::*};

#[derive(Reflect, Clone, Visit, Default, Debug)]
pub struct Vertex {
    buffer: VertexBuffer,
    #[visit(optional)]
    modifications_counter: u64,
    #[visit(optional)]
    layout_hash: u64,
}

impl Vertex {
    pub fn create_vertex_data(&self) -> Vec<u8> {
        self.buffer.create_packed_vertex_buffer_data()
    }

    pub fn get_vertex_layout(&self) -> VertexBufferLayout {
        let mut attributes = vec![];
        let mut accumulated_offset = 0;

        for (index, attribute_data) in self.buffer.attributes().values().enumerate() {
            attributes.push(VertexAttribute {
                format: attribute_data.desc.format,
                offset: accumulated_offset,
                shader_location: index as u32,
            });

            let format: RawVertexFormat = attribute_data.desc.format.into();

            accumulated_offset += format.size();
        }

        VertexBufferLayout {
            array_stride: 0,
            step_mode: VertexStepMode::Vertex,
            attributes,
        }
    }
}
