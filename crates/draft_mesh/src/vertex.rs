use draft_graphics::VertexFormat;
use fyrox_core::{reflect::*, visitor::*};

#[derive(Debug, Reflect, Visit, Clone)]
pub(crate) struct MeshAttributeData {
    pub(crate) attribute: MeshVertexAttribute,
    pub(crate) values: VertexAttributeValues,
}

#[derive(Debug, Reflect, Visit, Clone)]
pub enum VertexAttributeValues {
    /// One unsigned byte (u8). `u32` in shaders.
    Uint8(Vec<u8>),
    /// Two unsigned bytes (u8). `vec2<u32>` in shaders.
    Uint8x2(Vec<[u8; 2]>),
    /// Four unsigned bytes (u8). `vec4<u32>` in shaders.
    Uint8x4(Vec<[u8; 4]>),
    /// One signed byte (i8). `i32` in shaders.
    Sint8(Vec<i8>),
    /// Two signed bytes (i8). `vec2<i32>` in shaders.
    Sint8x2(Vec<[i8; 2]>),
    /// Four signed bytes (i8). `vec4<i32>` in shaders.
    Sint8x4(Vec<[i8; 4]>),
    /// One unsigned byte (u8). [0, 255] converted to float [0, 1] `f32` in shaders.
    Unorm8(Vec<u8>),
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm8x2(Vec<[u8; 2]>),
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4(Vec<[u8; 4]>),
    /// One signed byte (i8). [&minus;127, 127] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm8(Vec<i8>),
    /// Two signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm8x2(Vec<[i8; 2]>),
    /// Four signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
    Snorm8x4(Vec<[i8; 4]>),
    /// One unsigned short (u16). `u32` in shaders.
    Uint16(Vec<u16>),
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2(Vec<[u16; 2]>),
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4(Vec<[u16; 4]>),
    /// One signed short (i16). `i32` in shaders.
    Sint16(Vec<i16>),
    /// Two signed shorts (i16). `vec2<i32>` in shaders.
    Sint16x2(Vec<[i16; 2]>),
    /// Four signed shorts (i16). `vec4<i32>` in shaders.
    Sint16x4(Vec<[i16; 4]>),
    /// One unsigned short (u16). [0, 65535] converted to float [0, 1] `f32` in shaders.
    Unorm16(Vec<u16>),
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm16x2(Vec<[u16; 2]>),
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm16x4(Vec<[u16; 4]>),
    /// One signed short (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm16(Vec<i16>),
    /// Two signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm16x2(Vec<[i16; 2]>),
    /// Four signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
    Snorm16x4(Vec<[i16; 4]>),
    /// One single-precision float (f32). `f32` in shaders.
    Float32(Vec<f32>),
    /// Two single-precision floats (f32). `vec2<f32>` in shaders.
    Float32x2(Vec<[f32; 2]>),
    /// Three single-precision floats (f32). `vec3<f32>` in shaders.
    Float32x3(Vec<[f32; 3]>),
    /// Four single-precision floats (f32). `vec4<f32>` in shaders.
    Float32x4(Vec<[f32; 4]>),
    /// One unsigned int (u32). `u32` in shaders.
    Uint32(Vec<u32>),
    /// Two unsigned ints (u32). `vec2<u32>` in shaders.
    Uint32x2(Vec<[u32; 2]>),
    /// Three unsigned ints (u32). `vec3<u32>` in shaders.
    Uint32x3(Vec<[u32; 3]>),
    /// Four unsigned ints (u32). `vec4<u32>` in shaders.
    Uint32x4(Vec<[u32; 4]>),
    /// One signed int (i32). `i32` in shaders.
    Sint32(Vec<i32>),
    /// Two signed ints (i32). `vec2<i32>` in shaders.
    Sint32x2(Vec<[i32; 2]>),
    /// Three signed ints (i32). `vec3<i32>` in shaders.
    Sint32x3(Vec<[i32; 3]>),
    /// Four signed ints (i32). `vec4<i32>` in shaders.
    Sint32x4(Vec<[i32; 4]>),
    /// One double-precision float (f64). `f32` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64(Vec<f64>),
    /// Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x2(Vec<[f64; 2]>),
    /// Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x3(Vec<[f64; 3]>),
    /// Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x4(Vec<[f64; 4]>),
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
            VertexAttributeValues::Uint8(_) => VertexFormat::Uint8,
            VertexAttributeValues::Sint8(_) => VertexFormat::Sint8,
            VertexAttributeValues::Unorm8(_) => VertexFormat::Unorm8,
            VertexAttributeValues::Snorm8(_) => VertexFormat::Snorm8,
            VertexAttributeValues::Uint16(_) => VertexFormat::Uint16,
            VertexAttributeValues::Sint16(_) => VertexFormat::Sint16,
            VertexAttributeValues::Unorm16(_) => VertexFormat::Unorm16,
            VertexAttributeValues::Snorm16(_) => VertexFormat::Snorm16,
            VertexAttributeValues::Float64(_) => VertexFormat::Float64,
            VertexAttributeValues::Float64x2(_) => VertexFormat::Float64x2,
            VertexAttributeValues::Float64x3(_) => VertexFormat::Float64x3,
            VertexAttributeValues::Float64x4(_) => VertexFormat::Float64x4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Visit)]
pub struct MeshVertexAttribute {
    /// The _unique_ id of the vertex attribute. This will also determine sort ordering
    /// when generating vertex buffers. Built-in / standard attributes will use "close to zero"
    /// indices. When in doubt, use a random / very large u64 to avoid conflicts.
    pub id: MeshVertexAttributeId,

    /// The format of the vertex attribute.
    pub format: VertexFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Reflect, Visit)]
pub struct MeshVertexAttributeId(u64);
