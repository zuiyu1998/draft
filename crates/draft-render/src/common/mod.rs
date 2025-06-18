mod buffer;
pub mod cache;
pub mod color_target_state;
pub mod depth_stencil;
pub mod multisample_state;
pub mod primitive_state;
pub mod vertex_buffer_layout;

pub use buffer::*;
pub use cache::*;
pub use color_target_state::*;
pub use depth_stencil::*;
pub use multisample_state::*;
pub use primitive_state::*;
pub use vertex_buffer_layout::*;

use std::{
    collections::HashMap,
    num::{NonZeroU32, NonZeroU64},
};

pub use frame_graph::wgpu::{
    AstcBlock as RawAstcBlock, AstcChannel as RawAstcChannel,
    BindGroupLayoutEntry as RawBindGroupLayoutEntry, BindingType as RawBindingType,
    BufferBindingType as RawBufferBindingType,
    PipelineCompilationOptions as RawPipelineCompilationOptions,
    SamplerBindingType as RawSamplerBindingType, ShaderStages,
    StorageTextureAccess as RawStorageTextureAccess, TextureFormat as RawTextureFormat,
    TextureSampleType as RawTextureSampleType, TextureViewDimension as RawTextureViewDimension,
    VertexAttribute as RawVertexAttribute, VertexFormat as RawVertexFormat,
    VertexStepMode as RawVertexStepMode,
};
use fyrox_core::{reflect::*, visitor::*};

/// Advanced options for use when a pipeline is compiled
///
/// This implements `Default`, and for most users can be set to `Default::default()`
#[derive(Clone, Debug, Visit, Reflect, Default)]
pub struct PipelineCompilationOptions {
    /// Specifies the values of pipeline-overridable constants in the shader module.
    ///
    /// If an `@id` attribute was specified on the declaration,
    /// the key must be the pipeline constant ID as a decimal ASCII number; if not,
    /// the key must be the constant's identifier name.
    ///
    /// The value may represent any of WGSL's concrete scalar types.
    pub constants: HashMap<String, f64>,
    /// Whether workgroup scoped memory will be initialized with zero values for this stage.
    ///
    /// This is required by the WebGPU spec, but may have overhead which can be avoided
    /// for cross-platform applications
    pub zero_initialize_workgroup_memory: bool,
}

impl PipelineCompilationOptions {
    pub fn get_raw(&self) -> RawPipelineCompilationOptions {
        RawPipelineCompilationOptions {
            constants: &self.constants,
            zero_initialize_workgroup_memory: self.zero_initialize_workgroup_memory,
        }
    }
}

/// ASTC RGBA channel
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Visit, Reflect, Default)]
pub enum AstcChannel {
    /// 8 bit integer RGBA, [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    Unorm,
    /// 8 bit integer RGBA, Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    #[default]
    UnormSrgb,
    /// floating-point RGBA, linear-color float can be outside of the [0, 1] range.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_HDR`] must be enabled to use this channel.
    Hdr,
}

impl From<AstcChannel> for RawAstcChannel {
    fn from(value: AstcChannel) -> Self {
        match value {
            AstcChannel::Hdr => RawAstcChannel::Hdr,
            AstcChannel::Unorm => RawAstcChannel::Unorm,
            AstcChannel::UnormSrgb => RawAstcChannel::UnormSrgb,
        }
    }
}

/// ASTC block dimensions
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Visit, Reflect, Default)]
pub enum AstcBlock {
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    #[default]
    B4x4,
    /// 5x4 block compressed texture. 16 bytes per block (6.4 bit/px).
    B5x4,
    /// 5x5 block compressed texture. 16 bytes per block (5.12 bit/px).
    B5x5,
    /// 6x5 block compressed texture. 16 bytes per block (4.27 bit/px).
    B6x5,
    /// 6x6 block compressed texture. 16 bytes per block (3.56 bit/px).
    B6x6,
    /// 8x5 block compressed texture. 16 bytes per block (3.2 bit/px).
    B8x5,
    /// 8x6 block compressed texture. 16 bytes per block (2.67 bit/px).
    B8x6,
    /// 8x8 block compressed texture. 16 bytes per block (2 bit/px).
    B8x8,
    /// 10x5 block compressed texture. 16 bytes per block (2.56 bit/px).
    B10x5,
    /// 10x6 block compressed texture. 16 bytes per block (2.13 bit/px).
    B10x6,
    /// 10x8 block compressed texture. 16 bytes per block (1.6 bit/px).
    B10x8,
    /// 10x10 block compressed texture. 16 bytes per block (1.28 bit/px).
    B10x10,
    /// 12x10 block compressed texture. 16 bytes per block (1.07 bit/px).
    B12x10,
    /// 12x12 block compressed texture. 16 bytes per block (0.89 bit/px).
    B12x12,
}

impl From<AstcBlock> for RawAstcBlock {
    fn from(value: AstcBlock) -> Self {
        match value {
            AstcBlock::B4x4 => RawAstcBlock::B4x4,
            AstcBlock::B5x4 => RawAstcBlock::B5x4,
            AstcBlock::B5x5 => RawAstcBlock::B5x5,
            AstcBlock::B6x5 => RawAstcBlock::B6x5,
            AstcBlock::B6x6 => RawAstcBlock::B6x6,
            AstcBlock::B8x5 => RawAstcBlock::B8x5,
            AstcBlock::B8x6 => RawAstcBlock::B8x6,
            AstcBlock::B8x8 => RawAstcBlock::B8x8,
            AstcBlock::B10x5 => RawAstcBlock::B10x5,
            AstcBlock::B10x6 => RawAstcBlock::B10x6,
            AstcBlock::B10x8 => RawAstcBlock::B10x8,
            AstcBlock::B10x10 => RawAstcBlock::B10x10,
            AstcBlock::B12x10 => RawAstcBlock::B12x10,
            AstcBlock::B12x12 => RawAstcBlock::B12x12,
        }
    }
}

/// Underlying texture data format.
///
/// If there is a conversion in the format (such as srgb -> linear), the conversion listed here is for
/// loading from texture in a shader. When writing to the texture, the opposite conversion takes place.
///
/// Corresponds to [WebGPU `GPUTextureFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureformat).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Reflect, Visit, Default)]
pub enum TextureFormat {
    // Normal 8 bit formats
    /// Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    #[default]
    R8Unorm,
    /// Red channel only. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    R8Snorm,
    /// Red channel only. 8 bit integer per channel. RawTextureFormatUnsigned in shader.
    R8Uint,
    /// Red channel only. 8 bit integer per channel. Signed in shader.
    R8Sint,

    // Normal 16 bit formats
    /// Red channel only. 16 bit integer per channel. Unsigned in shader.
    R16Uint,
    /// Red channel only. 16 bit integer per channel. Signed in shader.
    R16Sint,
    /// Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Unorm,
    /// Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Snorm,
    /// Red channel only. 16 bit float per channel. Float in shader.
    R16Float,
    /// Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rg8Unorm,
    /// Red and green channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rg8Snorm,
    /// Red and green channels. 8 bit integer per channel. Unsigned in shader.
    Rg8Uint,
    /// Red and green channels. 8 bit integer per channel. Signed in shader.
    Rg8Sint,

    // Normal 32 bit formats
    /// Red channel only. 32 bit integer per channel. Unsigned in shader.
    R32Uint,
    /// Red channel only. 32 bit integer per channel. Signed in shader.
    R32Sint,
    /// Red channel only. 32 bit float per channel. Float in shader.
    R32Float,
    /// Red and green channels. 16 bit integer per channel. Unsigned in shader.
    Rg16Uint,
    /// Red and green channels. 16 bit integer per channel. Signed in shader.
    Rg16Sint,
    /// Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Unorm,
    /// Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Snorm,
    /// Red and green channels. 16 bit float per channel. Float in shader.
    Rg16Float,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rgba8Unorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Rgba8UnormSrgb,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rgba8Snorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader.
    Rgba8Uint,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader.
    Rgba8Sint,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Bgra8Unorm,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Bgra8UnormSrgb,

    // Packed 32 bit formats
    /// Packed unsigned float with 9 bits mantisa for each RGB component, then a common 5 bits exponent
    Rgb9e5Ufloat,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. Unsigned in shader.
    Rgb10a2Uint,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. [0, 1023] ([0, 3] for alpha) converted to/from float [0, 1] in shader.
    Rgb10a2Unorm,
    /// Red, green, and blue channels. 11 bit float with no sign bit for RG channels. 10 bit float with no sign bit for blue channel. Float in shader.
    Rg11b10Ufloat,

    // Normal 64 bit formats
    /// Red channel only. 64 bit integer per channel. Unsigned in shader.
    ///
    /// [`Features::TEXTURE_INT64_ATOMIC`] must be enabled to use this texture format.
    R64Uint,
    /// Red and green channels. 32 bit integer per channel. Unsigned in shader.
    Rg32Uint,
    /// Red and green channels. 32 bit integer per channel. Signed in shader.
    Rg32Sint,
    /// Red and green channels. 32 bit float per channel. Float in shader.
    Rg32Float,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Unsigned in shader.
    Rgba16Uint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Signed in shader.
    Rgba16Sint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Unorm,
    /// Red, green, blue, and alpha. 16 bit integer per channel. [0, 65535] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Snorm,
    /// Red, green, blue, and alpha channels. 16 bit float per channel. Float in shader.
    Rgba16Float,

    // Normal 128 bit formats
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Unsigned in shader.
    Rgba32Uint,
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Signed in shader.
    Rgba32Sint,
    /// Red, green, blue, and alpha channels. 32 bit float per channel. Float in shader.
    Rgba32Float,

    // Depth and stencil formats
    /// Stencil format with 8 bit integer stencil.
    Stencil8,
    /// Special depth format with 16 bit integer depth.
    Depth16Unorm,
    /// Special depth format with at least 24 bit integer depth.
    Depth24Plus,
    /// Special depth/stencil format with at least 24 bit integer depth and 8 bits integer stencil.
    Depth24PlusStencil8,
    /// Special depth format with 32 bit floating point depth.
    Depth32Float,
    /// Special depth/stencil format with 32 bit floating point depth and 8 bits integer stencil.
    ///
    /// [`Features::DEPTH32FLOAT_STENCIL8`] must be enabled to use this texture format.
    Depth32FloatStencil8,

    /// YUV 4:2:0 chroma subsampled format.
    ///
    /// Contains two planes:
    /// - 0: Single 8 bit channel luminance.
    /// - 1: Dual 8 bit channel chrominance at half width and half height.
    ///
    /// Valid view formats for luminance are [`TextureFormat::R8Unorm`].
    ///
    /// Valid view formats for chrominance are [`TextureFormat::Rg8Unorm`].
    ///
    /// Width and height must be even.
    ///
    /// [`Features::TEXTURE_FORMAT_NV12`] must be enabled to use this texture format.
    NV12,

    // Compressed textures usable with `TEXTURE_COMPRESSION_BC` feature. `TEXTURE_COMPRESSION_SLICED_3D` is required to use with 3D textures.
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// [0, 63] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc1RgbaUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// Srgb-color [0, 63] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc1RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// [0, 63] ([0, 15] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc2RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc2RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// [0, 63] ([0, 255] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc3RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc3RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc4RUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc4RSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc5RgUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc5RgSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit unsigned float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc6hRgbUfloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit signed float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc6hRgbFloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc7RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc7RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// [0, 255] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// Srgb-color [0, 255] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1UnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Snorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Snorm,
    /// block compressed texture. 16 bytes per block.
    ///
    /// Features [`TEXTURE_COMPRESSION_ASTC`] or [`TEXTURE_COMPRESSION_ASTC_HDR`]
    /// must be enabled to use this texture format.
    ///
    /// [`TEXTURE_COMPRESSION_ASTC`]: Features::TEXTURE_COMPRESSION_ASTC
    /// [`TEXTURE_COMPRESSION_ASTC_HDR`]: Features::TEXTURE_COMPRESSION_ASTC_HDR
    Astc {
        /// compressed block dimensions
        block: AstcBlock,
        /// ASTC RGBA channel
        channel: AstcChannel,
    },
}

impl From<TextureFormat> for RawTextureFormat {
    fn from(value: TextureFormat) -> Self {
        match value {
            TextureFormat::Astc { block, channel } => RawTextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
            TextureFormat::Bc1RgbaUnorm => RawTextureFormat::Bc1RgbaUnorm,
            TextureFormat::Bc1RgbaUnormSrgb => RawTextureFormat::Bc1RgbaUnormSrgb,
            TextureFormat::Bc2RgbaUnorm => RawTextureFormat::Bc2RgbaUnorm,
            TextureFormat::Bc2RgbaUnormSrgb => RawTextureFormat::Bc2RgbaUnormSrgb,
            TextureFormat::Bc3RgbaUnorm => RawTextureFormat::Bc3RgbaUnorm,
            TextureFormat::Bc3RgbaUnormSrgb => RawTextureFormat::Bc3RgbaUnormSrgb,
            TextureFormat::Bc4RSnorm => RawTextureFormat::Bc4RSnorm,
            TextureFormat::Bc4RUnorm => RawTextureFormat::Bc4RUnorm,
            TextureFormat::Bc5RgSnorm => RawTextureFormat::Bc5RgSnorm,
            TextureFormat::Bc5RgUnorm => RawTextureFormat::Bc5RgUnorm,
            TextureFormat::Bc6hRgbFloat => RawTextureFormat::Bc6hRgbFloat,
            TextureFormat::Bc6hRgbUfloat => RawTextureFormat::Bc6hRgbUfloat,
            TextureFormat::Bc7RgbaUnorm => RawTextureFormat::Bc7RgbaUnorm,
            TextureFormat::Bc7RgbaUnormSrgb => RawTextureFormat::Bc7RgbaUnormSrgb,
            TextureFormat::Bgra8Unorm => RawTextureFormat::Bgra8Unorm,
            TextureFormat::Bgra8UnormSrgb => RawTextureFormat::Bgra8UnormSrgb,
            TextureFormat::Depth16Unorm => RawTextureFormat::Depth16Unorm,
            TextureFormat::Depth24Plus => RawTextureFormat::Depth24Plus,
            TextureFormat::Depth24PlusStencil8 => RawTextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32Float => RawTextureFormat::Depth32Float,
            TextureFormat::Depth32FloatStencil8 => RawTextureFormat::Depth32FloatStencil8,
            TextureFormat::EacR11Snorm => RawTextureFormat::EacR11Snorm,
            TextureFormat::EacR11Unorm => RawTextureFormat::EacR11Unorm,
            TextureFormat::EacRg11Snorm => RawTextureFormat::EacRg11Snorm,
            TextureFormat::EacRg11Unorm => RawTextureFormat::EacRg11Unorm,
            TextureFormat::Etc2Rgb8A1Unorm => RawTextureFormat::Etc2Rgb8A1Unorm,
            TextureFormat::Etc2Rgb8A1UnormSrgb => RawTextureFormat::Etc2Rgb8A1UnormSrgb,
            TextureFormat::Etc2Rgb8Unorm => RawTextureFormat::Etc2Rgb8Unorm,
            TextureFormat::Etc2Rgb8UnormSrgb => RawTextureFormat::Etc2Rgb8UnormSrgb,
            TextureFormat::Etc2Rgba8Unorm => RawTextureFormat::Etc2Rgba8Unorm,
            TextureFormat::Etc2Rgba8UnormSrgb => RawTextureFormat::Etc2Rgba8UnormSrgb,
            TextureFormat::NV12 => RawTextureFormat::NV12,
            TextureFormat::R16Float => RawTextureFormat::R16Float,
            TextureFormat::R16Sint => RawTextureFormat::R16Sint,
            TextureFormat::R16Snorm => RawTextureFormat::R16Snorm,
            TextureFormat::R16Uint => RawTextureFormat::R16Uint,
            TextureFormat::R16Unorm => RawTextureFormat::R16Unorm,
            TextureFormat::R32Float => RawTextureFormat::R32Float,
            TextureFormat::R32Sint => RawTextureFormat::R32Sint,
            TextureFormat::R32Uint => RawTextureFormat::R32Uint,
            TextureFormat::R64Uint => RawTextureFormat::R64Uint,
            TextureFormat::R8Sint => RawTextureFormat::R8Sint,
            TextureFormat::R8Snorm => RawTextureFormat::R8Snorm,
            TextureFormat::R8Uint => RawTextureFormat::R8Uint,
            TextureFormat::R8Unorm => RawTextureFormat::R8Unorm,
            TextureFormat::Rg11b10Ufloat => RawTextureFormat::Rg11b10Ufloat,
            TextureFormat::Rg16Float => RawTextureFormat::Rg16Float,
            TextureFormat::Rg16Sint => RawTextureFormat::Rg16Sint,
            TextureFormat::Rg16Snorm => RawTextureFormat::Rg16Snorm,
            TextureFormat::Rg16Uint => RawTextureFormat::Rg16Uint,
            TextureFormat::Rg16Unorm => RawTextureFormat::Rg16Unorm,
            TextureFormat::Rg32Float => RawTextureFormat::Rg32Float,
            TextureFormat::Rg32Sint => RawTextureFormat::Rg32Sint,
            TextureFormat::Rg32Uint => RawTextureFormat::Rg32Uint,
            TextureFormat::Rg8Sint => RawTextureFormat::Rg8Sint,
            TextureFormat::Rg8Snorm => RawTextureFormat::Rg8Snorm,
            TextureFormat::Rg8Uint => RawTextureFormat::Rg8Uint,
            TextureFormat::Rg8Unorm => RawTextureFormat::Rg8Unorm,
            TextureFormat::Rgb10a2Uint => RawTextureFormat::Rgb10a2Uint,
            TextureFormat::Rgb10a2Unorm => RawTextureFormat::Rgb10a2Unorm,
            TextureFormat::Rgb9e5Ufloat => RawTextureFormat::Rgb9e5Ufloat,
            TextureFormat::Rgba16Float => RawTextureFormat::Rgba16Float,
            TextureFormat::Rgba16Sint => RawTextureFormat::Rgba16Float,
            TextureFormat::Rgba16Snorm => RawTextureFormat::Rgba16Snorm,
            TextureFormat::Rgba16Uint => RawTextureFormat::Rgba16Uint,
            TextureFormat::Rgba16Unorm => RawTextureFormat::Rgba16Unorm,
            TextureFormat::Rgba32Float => RawTextureFormat::Rgba32Float,
            TextureFormat::Rgba32Sint => RawTextureFormat::Rgba32Sint,
            TextureFormat::Rgba32Uint => RawTextureFormat::Rgba32Uint,
            TextureFormat::Rgba8Sint => RawTextureFormat::Rgba8Sint,
            TextureFormat::Rgba8Snorm => RawTextureFormat::Rgba8Snorm,
            TextureFormat::Rgba8Uint => RawTextureFormat::Rgba8Uint,
            TextureFormat::Rgba8Unorm => RawTextureFormat::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb => RawTextureFormat::Rgba8UnormSrgb,
            TextureFormat::Stencil8 => RawTextureFormat::Stencil8,
        }
    }
}

/// Specific type of a sample in a texture binding.
///
/// For use in [`BindingType::StorageTexture`].
///
/// Corresponds to [WebGPU `GPUStorageTextureAccess`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpustoragetextureaccess).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect, Visit, Default)]
pub enum StorageTextureAccess {
    /// The texture can only be written in the shader and it:
    /// - may or may not be annotated with `write` (WGSL).
    /// - must be annotated with `writeonly` (GLSL).
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) writeonly uniform image2D myStorageImage;
    /// ```
    WriteOnly,
    /// The texture can only be read in the shader and it must be annotated with `read` (WGSL) or
    /// `readonly` (GLSL).
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode. This is a native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, read>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) readonly uniform image2D myStorageImage;
    /// ```
    ReadOnly,
    /// The texture can be both read and written in the shader and must be annotated with
    /// `read_write` in WGSL.
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode.  This is a nonstandard, native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, read_write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) uniform image2D myStorageImage;
    /// ```
    ReadWrite,
    /// The texture can be both read and written in the shader via atomics and must be annotated
    /// with `read_write` in WGSL.
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode.  This is a nonstandard, native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32uint, atomic>;
    /// ```
    #[default]
    Atomic,
}

impl From<StorageTextureAccess> for RawStorageTextureAccess {
    fn from(value: StorageTextureAccess) -> Self {
        match value {
            StorageTextureAccess::Atomic => RawStorageTextureAccess::Atomic,
            StorageTextureAccess::ReadOnly => RawStorageTextureAccess::ReadOnly,
            StorageTextureAccess::ReadWrite => RawStorageTextureAccess::ReadWrite,
            StorageTextureAccess::WriteOnly => RawStorageTextureAccess::WriteOnly,
        }
    }
}

/// Dimensions of a particular texture view.
///
/// Corresponds to [WebGPU `GPUTextureViewDimension`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureviewdimension).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Reflect, Visit)]
pub enum TextureViewDimension {
    /// A one dimensional texture. `texture_1d` in WGSL and `texture1D` in GLSL.
    D1,
    /// A two dimensional texture. `texture_2d` in WGSL and `texture2D` in GLSL.
    #[default]
    D2,
    /// A two dimensional array texture. `texture_2d_array` in WGSL and `texture2DArray` in GLSL.
    D2Array,
    /// A cubemap texture. `texture_cube` in WGSL and `textureCube` in GLSL.
    Cube,
    /// A cubemap array texture. `texture_cube_array` in WGSL and `textureCubeArray` in GLSL.
    CubeArray,
    /// A three dimensional texture. `texture_3d` in WGSL and `texture3D` in GLSL.
    D3,
}

impl From<TextureViewDimension> for RawTextureViewDimension {
    fn from(value: TextureViewDimension) -> Self {
        match value {
            TextureViewDimension::Cube => RawTextureViewDimension::Cube,
            TextureViewDimension::D1 => RawTextureViewDimension::D1,
            TextureViewDimension::CubeArray => RawTextureViewDimension::CubeArray,
            TextureViewDimension::D2 => RawTextureViewDimension::D2,
            TextureViewDimension::D2Array => RawTextureViewDimension::D2Array,
            TextureViewDimension::D3 => RawTextureViewDimension::D3,
        }
    }
}

/// Specific type of a sample in a texture binding.
///
/// Corresponds to [WebGPU `GPUTextureSampleType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputexturesampletype).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect, Visit, Default)]
pub enum TextureSampleType {
    /// Sampling returns floats.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<f32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2D t;
    /// ```
    Float {
        /// If this is `false`, the texture can't be sampled with
        /// a filtering sampler.
        ///
        /// Even if this is `true`, it's possible to sample with
        /// a **non-filtering** sampler.
        filterable: bool,
    },
    /// Sampling does the depth reference comparison.
    ///
    /// This is also compatible with a non-filtering sampler.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_depth_2d;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2DShadow t;
    /// ```
    Depth,
    /// Sampling returns signed integers.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<i32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform itexture2D t;
    /// ```
    Sint,
    /// Sampling returns unsigned integers.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<u32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform utexture2D t;
    /// ```
    #[default]
    Uint,
}

impl From<TextureSampleType> for RawTextureSampleType {
    fn from(value: TextureSampleType) -> Self {
        match value {
            TextureSampleType::Depth => RawTextureSampleType::Depth,
            TextureSampleType::Sint => RawTextureSampleType::Sint,
            TextureSampleType::Float { filterable } => RawTextureSampleType::Float { filterable },
            TextureSampleType::Uint => RawTextureSampleType::Uint,
        }
    }
}

/// Specific type of a sampler binding.
///
/// For use in [`BindingType::Sampler`].
///
/// Corresponds to [WebGPU `GPUSamplerBindingType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpusamplerbindingtype).
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect, Visit, Default)]
pub enum SamplerBindingType {
    /// The sampling result is produced based on more than a single color sample from a texture,
    /// e.g. when bilinear interpolation is enabled.
    #[default]
    Filtering,
    /// The sampling result is produced based on a single color sample from a texture.
    NonFiltering,
    /// Use as a comparison sampler instead of a normal sampler.
    /// For more info take a look at the analogous functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Sampler_Object#Comparison_mode>.
    Comparison,
}

impl From<SamplerBindingType> for RawSamplerBindingType {
    fn from(value: SamplerBindingType) -> Self {
        match value {
            SamplerBindingType::Comparison => RawSamplerBindingType::Comparison,
            SamplerBindingType::Filtering => RawSamplerBindingType::Filtering,
            SamplerBindingType::NonFiltering => RawSamplerBindingType::NonFiltering,
        }
    }
}

/// Specific type of a buffer binding.
///
/// Corresponds to [WebGPU `GPUBufferBindingType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpubufferbindingtype).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Reflect, Visit)]
pub enum BufferBindingType {
    /// A buffer for uniform values.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// struct Globals {
    ///     a_uniform: vec2<f32>,
    ///     another_uniform: vec2<f32>,
    /// }
    /// @group(0) @binding(0)
    /// var<uniform> globals: Globals;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(std140, binding = 0)
    /// uniform Globals {
    ///     vec2 aUniform;
    ///     vec2 anotherUniform;
    /// };
    /// ```
    #[default]
    Uniform,
    /// A storage buffer.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var<storage, read_write> my_element: array<vec4<f32>>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout (set=0, binding=0) buffer myStorageBuffer {
    ///     vec4 myElement[];
    /// };
    /// ```
    Storage {
        /// If `true`, the buffer can only be read in the shader,
        /// and it:
        /// - may or may not be annotated with `read` (WGSL).
        /// - must be annotated with `readonly` (GLSL).
        ///
        /// Example WGSL syntax:
        /// ```rust,ignore
        /// @group(0) @binding(0)
        /// var<storage, read> my_element: array<vec4<f32>>;
        /// ```
        ///
        /// Example GLSL syntax:
        /// ```cpp,ignore
        /// layout (set=0, binding=0) readonly buffer myStorageBuffer {
        ///     vec4 myElement[];
        /// };
        /// ```
        read_only: bool,
    },
}

impl From<BufferBindingType> for RawBufferBindingType {
    fn from(value: BufferBindingType) -> Self {
        match value {
            BufferBindingType::Storage { read_only } => RawBufferBindingType::Storage { read_only },
            BufferBindingType::Uniform => RawBufferBindingType::Uniform,
        }
    }
}

/// Specific type of a binding.
///
/// For use in [`BindGroupLayoutEntry`].
///
/// Corresponds to WebGPU's mutually exclusive fields within [`GPUBindGroupLayoutEntry`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpubindgrouplayoutentry).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect, Visit, Default)]
pub enum BindingType {
    /// A buffer binding.
    ///
    /// Corresponds to [WebGPU `GPUBufferBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpubufferbindinglayout).
    Buffer {
        /// Sub-type of the buffer binding.
        ty: BufferBindingType,

        /// Indicates that the binding has a dynamic offset.
        ///
        /// One offset must be passed to [`RenderPass::set_bind_group`][RPsbg]
        /// for each dynamic binding in increasing order of binding number.
        ///
        /// [RPsbg]: ../wgpu/struct.RenderPass.html#method.set_bind_group
        has_dynamic_offset: bool,

        /// The minimum size for a [`BufferBinding`] matching this entry, in bytes.
        ///
        /// If this is `Some(size)`:
        ///
        /// - When calling [`create_bind_group`], the resource at this bind point
        ///   must be a [`BindingResource::Buffer`] whose effective size is at
        ///   least `size`.
        ///
        /// - When calling [`create_render_pipeline`] or [`create_compute_pipeline`],
        ///   `size` must be at least the [minimum buffer binding size] for the
        ///   shader module global at this bind point: large enough to hold the
        ///   global's value, along with one element of a trailing runtime-sized
        ///   array, if present.
        ///
        /// If this is `None`:
        ///
        /// - Each draw or dispatch command checks that the buffer range at this
        ///   bind point satisfies the [minimum buffer binding size].
        ///
        /// [`BufferBinding`]: ../wgpu/struct.BufferBinding.html
        /// [`create_bind_group`]: ../wgpu/struct.Device.html#method.create_bind_group
        /// [`BindingResource::Buffer`]: ../wgpu/enum.BindingResource.html#variant.Buffer
        /// [minimum buffer binding size]: https://www.w3.org/TR/webgpu/#minimum-buffer-binding-size
        /// [`create_render_pipeline`]: ../wgpu/struct.Device.html#method.create_render_pipeline
        /// [`create_compute_pipeline`]: ../wgpu/struct.Device.html#method.create_compute_pipeline
        min_binding_size: Option<u64>,
    },
    /// A sampler that can be used to sample a texture.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var s: sampler;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform sampler s;
    /// ```
    ///
    /// Corresponds to [WebGPU `GPUSamplerBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpusamplerbindinglayout).
    Sampler(SamplerBindingType),
    /// A texture binding.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<f32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2D t;
    /// ```
    ///
    /// Corresponds to [WebGPU `GPUTextureBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gputexturebindinglayout).
    Texture {
        /// Sample type of the texture binding.
        sample_type: TextureSampleType,
        /// Dimension of the texture view that is going to be sampled.
        view_dimension: TextureViewDimension,
        /// True if the texture has a sample count greater than 1. If this is true,
        /// the texture must be declared as `texture_multisampled_2d` or
        /// `texture_depth_multisampled_2d` in the shader, and read using `textureLoad`.
        multisampled: bool,
    },
    /// A storage texture.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) writeonly uniform image2D myStorageImage;
    /// ```
    /// Note that the texture format must be specified in the shader, along with the
    /// access mode. For WGSL, the format must be one of the enumerants in the list
    /// of [storage texel formats](https://gpuweb.github.io/gpuweb/wgsl/#storage-texel-formats).
    ///
    /// Corresponds to [WebGPU `GPUStorageTextureBindingLayout`](
    /// https://gpuweb.github.io/gpuweb/#dictdef-gpustoragetexturebindinglayout).
    StorageTexture {
        /// Allowed access to this texture.
        access: StorageTextureAccess,
        /// Format of the texture.
        format: TextureFormat,
        /// Dimension of the texture view that is going to be sampled.
        view_dimension: TextureViewDimension,
    },

    /// A ray-tracing acceleration structure binding.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var as: acceleration_structure;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform accelerationStructureEXT as;
    /// ```
    #[default]
    AccelerationStructure,
}

#[derive(Debug, Default, Reflect, Visit, Clone, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutEntry {
    /// Binding index. Must match shader index and be unique inside a BindGroupLayout. A binding
    /// of index 1, would be described as `layout(set = 0, binding = 1) uniform` in shaders.
    pub binding: u32,
    /// Which shader stages can see this binding.
    pub visibility: u32,
    /// The type of the binding
    pub ty: BindingType,
    /// If this value is Some, indicates this entry is an array. Array size must be 1 or greater.
    ///
    /// If this value is Some and `ty` is `BindingType::Texture`, [`Features::TEXTURE_BINDING_ARRAY`] must be supported.
    ///
    /// If this value is Some and `ty` is any other variant, bind group creation will fail.
    pub count: Option<u32>,
}

impl From<BindingType> for RawBindingType {
    fn from(value: BindingType) -> Self {
        match value {
            BindingType::AccelerationStructure => RawBindingType::AccelerationStructure,
            BindingType::Buffer {
                ty,
                has_dynamic_offset,
                min_binding_size,
            } => RawBindingType::Buffer {
                ty: ty.into(),
                has_dynamic_offset,
                min_binding_size: min_binding_size
                    .and_then(|value| NonZeroU64::try_from(value).ok()),
            },
            BindingType::Sampler(value) => RawBindingType::Sampler(value.into()),
            BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => RawBindingType::Texture {
                sample_type: sample_type.into(),
                view_dimension: view_dimension.into(),
                multisampled,
            },
            BindingType::StorageTexture {
                access,
                format,
                view_dimension,
            } => RawBindingType::StorageTexture {
                access: access.into(),
                format: format.into(),
                view_dimension: view_dimension.into(),
            },
        }
    }
}

impl From<BindGroupLayoutEntry> for RawBindGroupLayoutEntry {
    fn from(value: BindGroupLayoutEntry) -> Self {
        RawBindGroupLayoutEntry {
            binding: value.binding,
            visibility: ShaderStages::from_bits(value.visibility).unwrap_or(ShaderStages::NONE),
            ty: value.ty.into(),
            count: value
                .count
                .and_then(|value| NonZeroU32::try_from(value).ok()),
        }
    }
}
