use std::ops::{Deref, DerefMut};

use super::VertexBuffer;
use crate::gfx_base::{VertexAttribute, VertexBufferLayout, VertexStepMode};

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
    pub fn modify(&mut self) -> VertexModifier {
        VertexModifier {
            vertex: self,
            need_update: false,
        }
    }

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

            accumulated_offset += attribute_data.desc.size();
        }

        VertexBufferLayout {
            array_stride: 0,
            step_mode: VertexStepMode::Vertex,
            attributes,
        }
    }
}

pub struct VertexModifier<'a> {
    vertex: &'a mut Vertex,
    need_update: bool,
}

impl VertexModifier<'_> {
    pub fn set_need_update(&mut self, value: bool) {
        self.need_update = value;
    }
}

impl Deref for VertexModifier<'_> {
    type Target = VertexBuffer;

    fn deref(&self) -> &Self::Target {
        &self.vertex.buffer
    }
}

impl DerefMut for VertexModifier<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vertex.buffer
    }
}
