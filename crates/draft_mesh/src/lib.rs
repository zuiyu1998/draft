mod vertex;

use draft_core::RenderResource;
pub use vertex::*;

use draft_graphics::{PrimitiveTopology, VertexAttribute, VertexFormat};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

pub type MeshResource = Resource<Mesh>;

#[derive(Error, Debug, Clone)]
pub enum MeshAccessError {}

#[derive(Debug, Reflect, Visit, Clone)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

#[derive(Debug, Reflect, Clone)]
pub struct Mesh {
    primitive_topology: PrimitiveTopology,
    attributes: HashMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,
    modifications_counter: u64,
    indices_modifications_counter: u64,
    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new(PrimitiveTopology::default())
    }
}

impl Mesh {
    pub fn new(primitive_topology: PrimitiveTopology) -> Self {
        Mesh {
            primitive_topology,
            attributes: Default::default(),
            indices: Default::default(),
            cache_index: Default::default(),
            modifications_counter: 0,
            indices_modifications_counter: 0,
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
        for (index, data) in mesh_attributes.values().enumerate() {
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

        self.attributes
            .insert(attribute.id, MeshAttributeData { attribute, values });
    }
}

impl RenderResource for Mesh {
    fn get_cache_index(&self) -> &Arc<AtomicIndex> {
        &self.cache_index
    }
}

impl TypeUuidProvider for Mesh {
    fn type_uuid() -> Uuid {
        uuid!("3930d4ce-f524-4420-b8c8-459b5e427e93")
    }
}

impl Visit for Mesh {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        Ok(())
    }
}

impl ResourceData for Mesh {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
