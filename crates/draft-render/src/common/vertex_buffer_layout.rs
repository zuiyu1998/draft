use frame_graph::wgpu::{
    BufferAddress, ShaderLocation, VertexAttribute as RawVertexAttribute,
    VertexFormat as RawVertexFormat, VertexStepMode as RawVertexStepMode,
};
use fyrox_core::{reflect::*, visitor::*};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Visit, Reflect, Default)]
pub enum VertexFormat {
    /// One unsigned byte (u8). `u32` in shaders.
    #[default]
    Uint8 = 0,
    /// Two unsigned bytes (u8). `vec2<u32>` in shaders.
    Uint8x2 = 1,
    /// Four unsigned bytes (u8). `vec4<u32>` in shaders.
    Uint8x4 = 2,
    /// One signed byte (i8). `i32` in shaders.
    Sint8 = 3,
    /// Two signed bytes (i8). `vec2<i32>` in shaders.
    Sint8x2 = 4,
    /// Four signed bytes (i8). `vec4<i32>` in shaders.
    Sint8x4 = 5,
    /// One unsigned byte (u8). [0, 255] converted to float [0, 1] `f32` in shaders.
    Unorm8 = 6,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm8x2 = 7,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4 = 8,
    /// One signed byte (i8). [-127, 127] converted to float [-1, 1] `f32` in shaders.
    Snorm8 = 9,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm8x2 = 10,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm8x4 = 11,
    /// One unsigned short (u16). `u32` in shaders.
    Uint16 = 12,
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2 = 13,
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4 = 14,
    /// One signed short (u16). `i32` in shaders.
    Sint16 = 15,
    /// Two signed shorts (i16). `vec2<i32>` in shaders.
    Sint16x2 = 16,
    /// Four signed shorts (i16). `vec4<i32>` in shaders.
    Sint16x4 = 17,
    /// One unsigned short (u16). [0, 65535] converted to float [0, 1] `f32` in shaders.
    Unorm16 = 18,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm16x2 = 19,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm16x4 = 20,
    /// One signed short (i16). [-32767, 32767] converted to float [-1, 1] `f32` in shaders.
    Snorm16 = 21,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2<f32>` in shaders.
    Snorm16x2 = 22,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4<f32>` in shaders.
    Snorm16x4 = 23,
    /// One half-precision float (no Rust equiv). `f32` in shaders.
    Float16 = 24,
    /// Two half-precision floats (no Rust equiv). `vec2<f32>` in shaders.
    Float16x2 = 25,
    /// Four half-precision floats (no Rust equiv). `vec4<f32>` in shaders.
    Float16x4 = 26,
    /// One single-precision float (f32). `f32` in shaders.
    Float32 = 27,
    /// Two single-precision floats (f32). `vec2<f32>` in shaders.
    Float32x2 = 28,
    /// Three single-precision floats (f32). `vec3<f32>` in shaders.
    Float32x3 = 29,
    /// Four single-precision floats (f32). `vec4<f32>` in shaders.
    Float32x4 = 30,
    /// One unsigned int (u32). `u32` in shaders.
    Uint32 = 31,
    /// Two unsigned ints (u32). `vec2<u32>` in shaders.
    Uint32x2 = 32,
    /// Three unsigned ints (u32). `vec3<u32>` in shaders.
    Uint32x3 = 33,
    /// Four unsigned ints (u32). `vec4<u32>` in shaders.
    Uint32x4 = 34,
    /// One signed int (i32). `i32` in shaders.
    Sint32 = 35,
    /// Two signed ints (i32). `vec2<i32>` in shaders.
    Sint32x2 = 36,
    /// Three signed ints (i32). `vec3<i32>` in shaders.
    Sint32x3 = 37,
    /// Four signed ints (i32). `vec4<i32>` in shaders.
    Sint32x4 = 38,
    /// One double-precision float (f64). `f32` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64 = 39,
    /// Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x2 = 40,
    /// Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x3 = 41,
    /// Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x4 = 42,
    /// Three unsigned 10-bit integers and one 2-bit integer, packed into a 32-bit integer (u32). [0, 1024] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm10_10_10_2 = 43,
    /// Four unsigned 8-bit integers, packed into a 32-bit integer (u32). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4Bgra = 44,
}

impl From<VertexFormat> for RawVertexFormat {
    fn from(value: VertexFormat) -> Self {
        match value {
            VertexFormat::Float16 => RawVertexFormat::Float16,
            VertexFormat::Float16x2 => RawVertexFormat::Float16x2,
            VertexFormat::Float16x4 => RawVertexFormat::Float16x4,
            VertexFormat::Float32 => RawVertexFormat::Float32,
            VertexFormat::Float32x2 => RawVertexFormat::Float32x2,
            VertexFormat::Float32x3 => RawVertexFormat::Float32x3,
            VertexFormat::Float32x4 => RawVertexFormat::Float32x4,
            VertexFormat::Float64 => RawVertexFormat::Float64,
            VertexFormat::Float64x2 => RawVertexFormat::Float64x2,
            VertexFormat::Float64x3 => RawVertexFormat::Float64x3,
            VertexFormat::Float64x4 => RawVertexFormat::Float64x4,
            VertexFormat::Sint16 => RawVertexFormat::Sint16,
            VertexFormat::Sint16x2 => RawVertexFormat::Sint16x2,
            VertexFormat::Sint16x4 => RawVertexFormat::Sint16x4,
            VertexFormat::Sint32 => RawVertexFormat::Sint32,
            VertexFormat::Sint32x2 => RawVertexFormat::Sint32x2,
            VertexFormat::Sint32x3 => RawVertexFormat::Sint32x3,
            VertexFormat::Sint32x4 => RawVertexFormat::Sint32x4,
            VertexFormat::Sint8 => RawVertexFormat::Sint8,
            VertexFormat::Sint8x2 => RawVertexFormat::Sint8x2,
            VertexFormat::Sint8x4 => RawVertexFormat::Sint8x4,
            VertexFormat::Snorm16 => RawVertexFormat::Snorm16,
            VertexFormat::Snorm16x2 => RawVertexFormat::Snorm16x2,
            VertexFormat::Snorm16x4 => RawVertexFormat::Snorm16x4,
            VertexFormat::Snorm8 => RawVertexFormat::Snorm8,
            VertexFormat::Snorm8x2 => RawVertexFormat::Snorm8x2,
            VertexFormat::Snorm8x4 => RawVertexFormat::Snorm8x4,
            VertexFormat::Uint16 => RawVertexFormat::Uint16,
            VertexFormat::Uint16x2 => RawVertexFormat::Uint16x2,
            VertexFormat::Uint16x4 => RawVertexFormat::Uint16x4,
            VertexFormat::Uint32 => RawVertexFormat::Uint32,
            VertexFormat::Uint32x2 => RawVertexFormat::Uint32x2,
            VertexFormat::Uint32x3 => RawVertexFormat::Uint32x3,
            VertexFormat::Uint32x4 => RawVertexFormat::Uint32x4,
            VertexFormat::Uint8 => RawVertexFormat::Uint8,
            VertexFormat::Uint8x2 => RawVertexFormat::Uint8x2,
            VertexFormat::Uint8x4 => RawVertexFormat::Uint8x4,
            VertexFormat::Unorm10_10_10_2 => RawVertexFormat::Unorm10_10_10_2,
            VertexFormat::Unorm16 => RawVertexFormat::Unorm16,
            VertexFormat::Unorm16x2 => RawVertexFormat::Unorm16x2,
            VertexFormat::Unorm16x4 => RawVertexFormat::Unorm16x4,
            VertexFormat::Unorm8 => RawVertexFormat::Unorm8,
            VertexFormat::Unorm8x2 => RawVertexFormat::Unorm8,
            VertexFormat::Unorm8x4 => RawVertexFormat::Unorm8x4,
            VertexFormat::Unorm8x4Bgra => RawVertexFormat::Unorm8x4Bgra,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Visit, Default)]
pub struct VertexAttribute {
    /// Format of the input
    pub format: VertexFormat,
    /// Byte offset of the start of the input
    pub offset: BufferAddress,
    /// Location for this input. Must match the location in the shader.
    pub shader_location: ShaderLocation,
}

impl<'a> From<&'a VertexAttribute> for RawVertexAttribute {
    fn from(value: &'a VertexAttribute) -> Self {
        Self {
            format: value.format.into(),
            offset: value.offset,
            shader_location: value.shader_location,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Visit, Reflect)]
pub enum VertexStepMode {
    /// Vertex data is advanced every vertex.
    #[default]
    Vertex = 0,
    /// Vertex data is advanced every instance.
    Instance = 1,
}

impl From<VertexStepMode> for RawVertexStepMode {
    fn from(value: VertexStepMode) -> Self {
        match value {
            VertexStepMode::Instance => RawVertexStepMode::Instance,
            VertexStepMode::Vertex => RawVertexStepMode::Vertex,
        }
    }
}

/// Describes how the vertex buffer is interpreted.
#[derive(Default, Clone, Debug, Hash, Eq, PartialEq, Reflect, Visit)]
pub struct VertexBufferLayout {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: Vec<VertexAttribute>,
}
