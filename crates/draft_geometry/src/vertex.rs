use draft_graphics::{VertexAttribute, VertexBufferLayout, VertexFormat};
use fxhash::FxHashSet;
use fyrox_core::{ImmutableString, reflect::*};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};
use thiserror::Error;

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
    pub(crate) attribute: GeometryhVertexAttribute,
    pub(crate) values: VertexAttributeValues,
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct GeometryhVertexAttribute {
    pub name: ImmutableString,
    pub id: GeometryVertexAttributeId,
    pub format: VertexFormat,
}

impl GeometryhVertexAttribute {
    pub fn new(name: &'static str, id: u64, format: VertexFormat) -> Self {
        Self {
            name: ImmutableString::new(name),
            id: GeometryVertexAttributeId(id),
            format,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Reflect, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryVertexAttributeId(u64);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GeometryVertexBufferLayout {
    pub(crate) attribute_ids: Vec<GeometryVertexAttributeId>,
    pub(crate) layout: VertexBufferLayout,
}

impl GeometryVertexBufferLayout {
    pub fn new(attribute_ids: Vec<GeometryVertexAttributeId>, layout: VertexBufferLayout) -> Self {
        Self {
            attribute_ids,
            layout,
        }
    }

    #[inline]
    pub fn contains(&self, attribute_id: impl Into<GeometryVertexAttributeId>) -> bool {
        self.attribute_ids.contains(&attribute_id.into())
    }

    #[inline]
    pub fn attribute_ids(&self) -> &[GeometryVertexAttributeId] {
        &self.attribute_ids
    }

    #[inline]
    pub fn layout(&self) -> &VertexBufferLayout {
        &self.layout
    }

    pub fn get_layout(
        &self,
        attribute_descriptors: &[VertexAttributeDescriptor],
    ) -> Result<VertexBufferLayout, MissingVertexAttributeError> {
        let mut attributes = Vec::with_capacity(attribute_descriptors.len());
        for attribute_descriptor in attribute_descriptors {
            if let Some(index) = self
                .attribute_ids
                .iter()
                .position(|id| *id == attribute_descriptor.id)
            {
                let layout_attribute = &self.layout.attributes[index];
                attributes.push(VertexAttribute {
                    format: layout_attribute.format,
                    offset: layout_attribute.offset,
                    shader_location: attribute_descriptor.shader_location,
                });
            } else {
                return Err(MissingVertexAttributeError {
                    id: attribute_descriptor.id,
                    name: attribute_descriptor.name,
                    pipeline_type: None,
                });
            }
        }

        Ok(VertexBufferLayout {
            array_stride: self.layout.array_stride,
            step_mode: self.layout.step_mode,
            attributes,
        })
    }
}

#[derive(Error, Debug)]
#[error(
    "Geometry is missing requested attribute: {name} ({id:?}, pipeline type: {pipeline_type:?})"
)]
pub struct MissingVertexAttributeError {
    pub pipeline_type: Option<&'static str>,
    id: GeometryVertexAttributeId,
    name: &'static str,
}

pub struct VertexAttributeDescriptor {
    pub shader_location: u32,
    pub id: GeometryVertexAttributeId,
    name: &'static str,
}

impl VertexAttributeDescriptor {
    pub const fn new(
        shader_location: u32,
        id: GeometryVertexAttributeId,
        name: &'static str,
    ) -> Self {
        Self {
            shader_location,
            id,
            name,
        }
    }
}

#[derive(Debug, Default)]
pub struct GeometryVertexBufferLayouts(FxHashSet<Arc<GeometryVertexBufferLayout>>);

impl GeometryVertexBufferLayouts {
    pub fn insert(&mut self, layout: GeometryVertexBufferLayout) -> GeometryVertexBufferLayoutRef {
        let layout = match self.0.get(&layout) {
            Some(layout) => layout.clone(),
            None => {
                let layout = Arc::new(layout.clone());
                self.0.insert(layout.clone());
                layout
            }
        };

        GeometryVertexBufferLayoutRef(layout)
    }
}

#[derive(Clone, Debug)]
pub struct GeometryVertexBufferLayoutRef(pub Arc<GeometryVertexBufferLayout>);

impl PartialEq for GeometryVertexBufferLayoutRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for GeometryVertexBufferLayoutRef {}

impl Hash for GeometryVertexBufferLayoutRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the address of the underlying data, so two layouts that share the same
        // `MeshVertexBufferLayout` will have the same hash.
        (Arc::as_ptr(&self.0) as usize).hash(state);
    }
}
