mod primitives;

pub use primitives::*;

use std::{collections::BTreeMap, error::Error, path::Path, sync::Arc};

use draft_graphics::VertexFormat;
use fyrox_core::{
    ImmutableString, TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*,
};
use fyrox_resource::ResourceData;

#[derive(Debug, Clone, Default, Reflect, Visit)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    #[default]
    TriangleList = 3,
    TriangleStrip = 4,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct MeshVertexAttribute {
    pub name: ImmutableString,
    pub id: MeshVertexAttributeId,
    pub format: VertexFormat,
}

impl MeshVertexAttribute {
    pub fn new(name: &'static str, id: u64, format: VertexFormat) -> Self {
        Self {
            name: ImmutableString::new(name),
            id: MeshVertexAttributeId(id),
            format,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
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

macro_rules! impl_from {
    ($from:ty, $variant:tt) => {
        impl From<Vec<$from>> for VertexAttributeValues {
            fn from(vec: Vec<$from>) -> Self {
                VertexAttributeValues::$variant(vec)
            }
        }
    };
}

impl_from!([f32; 3], Float32x3);
impl_from!([f32; 2], Float32x2);

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

impl Default for VertexAttributeValues {
    fn default() -> Self {
        VertexAttributeValues::Float32(vec![])
    }
}

#[derive(Debug, Clone, Default, Reflect)]
pub(crate) struct MeshAttributeData {
    pub(crate) attribute: MeshVertexAttribute,
    pub(crate) values: VertexAttributeValues,
}

#[derive(Debug, Clone, Copy, Default, Reflect, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshVertexAttributeId(u64);

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
    attributes: BTreeMap<MeshVertexAttributeId, MeshAttributeData>,
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

    pub fn attribute_position() -> MeshVertexAttribute {
        MeshVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x3)
    }

    pub fn attribute_normal() -> MeshVertexAttribute {
        MeshVertexAttribute::new("Vertex_Normal", 1, VertexFormat::Float32x3)
    }

    pub fn attribute_uv_0() -> MeshVertexAttribute {
        MeshVertexAttribute::new("Vertex_Uv", 2, VertexFormat::Float32x2)
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
        attribute: MeshVertexAttribute,
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
        attribute: MeshVertexAttribute,
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
