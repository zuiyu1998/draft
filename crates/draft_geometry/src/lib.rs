mod primitives;
mod vertex;

pub use primitives::*;
pub use vertex::*;

use std::{collections::BTreeMap, error::Error, path::Path, sync::Arc};

use draft_graphics::{PrimitiveTopology, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use fyrox_resource::{Resource, ResourceData};

pub type GeometryResource = Resource<Geometry>;

#[derive(Debug, Clone, Reflect)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U32(vec![])
    }
}

#[derive(Debug, Clone, Default, Reflect, TypeUuidProvider)]
#[type_uuid(id = "8a23a414-e66d-4e12-9628-92c6ab49c2f0")]
pub struct Geometry {
    primitive_topology: PrimitiveTopology,
    #[reflect(hidden)]
    attributes: BTreeMap<GeometryVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,

    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Geometry {
    pub fn new(primitive_topology: PrimitiveTopology) -> Self {
        Geometry {
            primitive_topology,
            attributes: Default::default(),
            indices: None,
            cache_index: Default::default(),
        }
    }

    pub fn get_mesh_vertex_buffer_layout(
        &self,
        mesh_vertex_buffer_layouts: &mut GeometryVertexBufferLayouts,
    ) -> GeometryVertexBufferLayoutRef {
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

        let layout = GeometryVertexBufferLayout {
            layout: VertexBufferLayout {
                array_stride: accumulated_offset,
                step_mode: VertexStepMode::Vertex,
                attributes,
            },
            attribute_ids,
        };
        mesh_vertex_buffer_layouts.insert(layout)
    }

    pub fn attribute_position() -> GeometryhVertexAttribute {
        GeometryhVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x3)
    }

    pub fn attribute_normal() -> GeometryhVertexAttribute {
        GeometryhVertexAttribute::new("Vertex_Normal", 1, VertexFormat::Float32x3)
    }

    pub fn attribute_uv_0() -> GeometryhVertexAttribute {
        GeometryhVertexAttribute::new("Vertex_Uv", 2, VertexFormat::Float32x2)
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
        attribute: GeometryhVertexAttribute,
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
        attribute: GeometryhVertexAttribute,
        values: impl Into<VertexAttributeValues>,
    ) -> Self {
        self.insert_attribute(attribute, values);
        self
    }
}

impl Visit for Geometry {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        // self.attributes.visit("Attributes", &mut region)?;

        Ok(())
    }
}

impl ResourceData for Geometry {
    fn type_uuid(&self) -> Uuid {
        <Geometry as fyrox_core::TypeUuidProvider>::type_uuid()
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
