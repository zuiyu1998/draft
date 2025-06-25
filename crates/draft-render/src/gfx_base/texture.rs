use super::{
    RawAstcBlock, RawAstcChannel, RawExtent3d, RawTextureDimension, RawTextureFormat,
    RawTextureUsages,
};
use fyrox_core::{reflect::*, visitor::*};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Reflect, Visit, Default)]
pub struct TextureUsages(u32);

bitflags::bitflags! {
    impl TextureUsages: u32 {
          //
        // ---- Start numbering at 1 << 0 ----
        //
        // WebGPU features:
        //
        /// Allows a texture to be the source in a [`CommandEncoder::copy_texture_to_buffer`] or
        /// [`CommandEncoder::copy_texture_to_texture`] operation.
        const COPY_SRC = 1 << 0;
        /// Allows a texture to be the destination in a  [`CommandEncoder::copy_buffer_to_texture`],
        /// [`CommandEncoder::copy_texture_to_texture`], or [`Queue::write_texture`] operation.
        const COPY_DST = 1 << 1;
        /// Allows a texture to be a [`BindingType::Texture`] in a bind group.
        const TEXTURE_BINDING = 1 << 2;
        /// Allows a texture to be a [`BindingType::StorageTexture`] in a bind group.
        const STORAGE_BINDING = 1 << 3;
        /// Allows a texture to be an output attachment of a render pass.
        const RENDER_ATTACHMENT = 1 << 4;

        //
        // ---- Restart Numbering for Native Features ---
        //
        // Native Features:
        //
        /// Allows a texture to be used with image atomics. Requires [`Features::TEXTURE_ATOMIC`].
        const STORAGE_ATOMIC = 1 << 16;
   }
}

impl From<TextureUsages> for RawTextureUsages {
    fn from(value: TextureUsages) -> Self {
        RawTextureUsages::from_bits(value.0).unwrap_or(RawTextureUsages::all())
    }
}

impl From<RawTextureUsages> for TextureUsages {
    fn from(value: RawTextureUsages) -> Self {
        TextureUsages::from_bits(value.bits()).unwrap_or(TextureUsages::all())
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Reflect, Visit, Default)]
pub enum TextureDimension {
    /// 1D texture
    D1,
    /// 2D texture
    #[default]
    D2,
    /// 3D texture
    D3,
}

impl From<TextureDimension> for RawTextureDimension {
    fn from(value: TextureDimension) -> Self {
        match value {
            TextureDimension::D1 => RawTextureDimension::D1,
            TextureDimension::D2 => RawTextureDimension::D2,
            TextureDimension::D3 => RawTextureDimension::D3,
        }
    }
}

impl From<RawTextureDimension> for TextureDimension {
    fn from(value: RawTextureDimension) -> Self {
        match value {
            RawTextureDimension::D1 => TextureDimension::D1,
            RawTextureDimension::D2 => TextureDimension::D2,
            RawTextureDimension::D3 => TextureDimension::D3,
        }
    }
}

/// Used to calculate the volume of an item.
pub trait Volume {
    fn volume(&self) -> usize;
}

impl Volume for Extent3d {
    /// Calculates the volume of the [`Extent3d`].
    fn volume(&self) -> usize {
        (self.width * self.height * self.depth_or_array_layers) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Reflect, Visit, Debug, Default)]
pub struct Extent3d {
    /// Width of the extent
    pub width: u32,
    /// Height of the extent
    pub height: u32,
    /// The depth of the extent or the number of array layers
    pub depth_or_array_layers: u32,
}

impl From<Extent3d> for RawExtent3d {
    fn from(value: Extent3d) -> Self {
        RawExtent3d {
            width: value.width,
            height: value.height,
            depth_or_array_layers: value.depth_or_array_layers,
        }
    }
}

impl From<RawExtent3d> for Extent3d {
    fn from(value: RawExtent3d) -> Self {
        Extent3d {
            width: value.width,
            height: value.height,
            depth_or_array_layers: value.depth_or_array_layers,
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

impl From<RawAstcChannel> for AstcChannel {
    fn from(value: RawAstcChannel) -> Self {
        match value {
            RawAstcChannel::Hdr => AstcChannel::Hdr,
            RawAstcChannel::Unorm => AstcChannel::Unorm,
            RawAstcChannel::UnormSrgb => AstcChannel::UnormSrgb,
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

impl From<RawAstcBlock> for AstcBlock {
    fn from(value: RawAstcBlock) -> Self {
        match value {
            RawAstcBlock::B4x4 => AstcBlock::B4x4,
            RawAstcBlock::B5x4 => AstcBlock::B5x4,
            RawAstcBlock::B5x5 => AstcBlock::B5x5,
            RawAstcBlock::B6x5 => AstcBlock::B6x5,
            RawAstcBlock::B6x6 => AstcBlock::B6x6,
            RawAstcBlock::B8x5 => AstcBlock::B8x5,
            RawAstcBlock::B8x6 => AstcBlock::B8x6,
            RawAstcBlock::B8x8 => AstcBlock::B8x8,
            RawAstcBlock::B10x5 => AstcBlock::B10x5,
            RawAstcBlock::B10x6 => AstcBlock::B10x6,
            RawAstcBlock::B10x8 => AstcBlock::B10x8,
            RawAstcBlock::B10x10 => AstcBlock::B10x10,
            RawAstcBlock::B12x10 => AstcBlock::B12x10,
            RawAstcBlock::B12x12 => AstcBlock::B12x12,
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

pub trait TextureFormatPixelInfo {
    /// Returns the size of a pixel in bytes of the format.
    fn pixel_size(&self) -> usize;
}

impl TextureFormatPixelInfo for TextureFormat {
    fn pixel_size(&self) -> usize {
        let info: RawTextureFormat = (*self).into();
        match info.block_dimensions() {
            (1, 1) => info.block_copy_size(None).unwrap() as usize,
            _ => panic!("Using pixel_size for compressed textures is invalid"),
        }
    }
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

impl From<RawTextureFormat> for TextureFormat {
    fn from(value: RawTextureFormat) -> Self {
        match value {
            RawTextureFormat::Astc { block, channel } => TextureFormat::Astc {
                block: block.into(),
                channel: channel.into(),
            },
            RawTextureFormat::Bc1RgbaUnorm => TextureFormat::Bc1RgbaUnorm,
            RawTextureFormat::Bc1RgbaUnormSrgb => TextureFormat::Bc1RgbaUnormSrgb,
            RawTextureFormat::Bc2RgbaUnorm => TextureFormat::Bc2RgbaUnorm,
            RawTextureFormat::Bc2RgbaUnormSrgb => TextureFormat::Bc2RgbaUnormSrgb,
            RawTextureFormat::Bc3RgbaUnorm => TextureFormat::Bc3RgbaUnorm,
            RawTextureFormat::Bc3RgbaUnormSrgb => TextureFormat::Bc3RgbaUnormSrgb,
            RawTextureFormat::Bc4RSnorm => TextureFormat::Bc4RSnorm,
            RawTextureFormat::Bc4RUnorm => TextureFormat::Bc4RUnorm,
            RawTextureFormat::Bc5RgSnorm => TextureFormat::Bc5RgSnorm,
            RawTextureFormat::Bc5RgUnorm => TextureFormat::Bc5RgUnorm,
            RawTextureFormat::Bc6hRgbFloat => TextureFormat::Bc6hRgbFloat,
            RawTextureFormat::Bc6hRgbUfloat => TextureFormat::Bc6hRgbUfloat,
            RawTextureFormat::Bc7RgbaUnorm => TextureFormat::Bc7RgbaUnorm,
            RawTextureFormat::Bc7RgbaUnormSrgb => TextureFormat::Bc7RgbaUnormSrgb,
            RawTextureFormat::Bgra8Unorm => TextureFormat::Bgra8Unorm,
            RawTextureFormat::Bgra8UnormSrgb => TextureFormat::Bgra8UnormSrgb,
            RawTextureFormat::Depth16Unorm => TextureFormat::Depth16Unorm,
            RawTextureFormat::Depth24Plus => TextureFormat::Depth24Plus,
            RawTextureFormat::Depth24PlusStencil8 => TextureFormat::Depth24PlusStencil8,
            RawTextureFormat::Depth32Float => TextureFormat::Depth32Float,
            RawTextureFormat::Depth32FloatStencil8 => TextureFormat::Depth32FloatStencil8,
            RawTextureFormat::EacR11Snorm => TextureFormat::EacR11Snorm,
            RawTextureFormat::EacR11Unorm => TextureFormat::EacR11Unorm,
            RawTextureFormat::EacRg11Snorm => TextureFormat::EacRg11Snorm,
            RawTextureFormat::EacRg11Unorm => TextureFormat::EacRg11Unorm,
            RawTextureFormat::Etc2Rgb8A1Unorm => TextureFormat::Etc2Rgb8A1Unorm,
            RawTextureFormat::Etc2Rgb8A1UnormSrgb => TextureFormat::Etc2Rgb8A1UnormSrgb,
            RawTextureFormat::Etc2Rgb8Unorm => TextureFormat::Etc2Rgb8Unorm,
            RawTextureFormat::Etc2Rgb8UnormSrgb => TextureFormat::Etc2Rgb8UnormSrgb,
            RawTextureFormat::Etc2Rgba8Unorm => TextureFormat::Etc2Rgba8Unorm,
            RawTextureFormat::Etc2Rgba8UnormSrgb => TextureFormat::Etc2Rgba8UnormSrgb,
            RawTextureFormat::NV12 => TextureFormat::NV12,
            RawTextureFormat::R16Float => TextureFormat::R16Float,
            RawTextureFormat::R16Sint => TextureFormat::R16Sint,
            RawTextureFormat::R16Snorm => TextureFormat::R16Snorm,
            RawTextureFormat::R16Uint => TextureFormat::R16Uint,
            RawTextureFormat::R16Unorm => TextureFormat::R16Unorm,
            RawTextureFormat::R32Float => TextureFormat::R32Float,
            RawTextureFormat::R32Sint => TextureFormat::R32Sint,
            RawTextureFormat::R32Uint => TextureFormat::R32Uint,
            RawTextureFormat::R64Uint => TextureFormat::R64Uint,
            RawTextureFormat::R8Sint => TextureFormat::R8Sint,
            RawTextureFormat::R8Snorm => TextureFormat::R8Snorm,
            RawTextureFormat::R8Uint => TextureFormat::R8Uint,
            RawTextureFormat::R8Unorm => TextureFormat::R8Unorm,
            RawTextureFormat::Rg11b10Ufloat => TextureFormat::Rg11b10Ufloat,
            RawTextureFormat::Rg16Float => TextureFormat::Rg16Float,
            RawTextureFormat::Rg16Sint => TextureFormat::Rg16Sint,
            RawTextureFormat::Rg16Snorm => TextureFormat::Rg16Snorm,
            RawTextureFormat::Rg16Uint => TextureFormat::Rg16Uint,
            RawTextureFormat::Rg16Unorm => TextureFormat::Rg16Unorm,
            RawTextureFormat::Rg32Float => TextureFormat::Rg32Float,
            RawTextureFormat::Rg32Sint => TextureFormat::Rg32Sint,
            RawTextureFormat::Rg32Uint => TextureFormat::Rg32Uint,
            RawTextureFormat::Rg8Sint => TextureFormat::Rg8Sint,
            RawTextureFormat::Rg8Snorm => TextureFormat::Rg8Snorm,
            RawTextureFormat::Rg8Uint => TextureFormat::Rg8Uint,
            RawTextureFormat::Rg8Unorm => TextureFormat::Rg8Unorm,
            RawTextureFormat::Rgb10a2Uint => TextureFormat::Rgb10a2Uint,
            RawTextureFormat::Rgb10a2Unorm => TextureFormat::Rgb10a2Unorm,
            RawTextureFormat::Rgb9e5Ufloat => TextureFormat::Rgb9e5Ufloat,
            RawTextureFormat::Rgba16Float => TextureFormat::Rgba16Float,
            RawTextureFormat::Rgba16Sint => TextureFormat::Rgba16Float,
            RawTextureFormat::Rgba16Snorm => TextureFormat::Rgba16Snorm,
            RawTextureFormat::Rgba16Uint => TextureFormat::Rgba16Uint,
            RawTextureFormat::Rgba16Unorm => TextureFormat::Rgba16Unorm,
            RawTextureFormat::Rgba32Float => TextureFormat::Rgba32Float,
            RawTextureFormat::Rgba32Sint => TextureFormat::Rgba32Sint,
            RawTextureFormat::Rgba32Uint => TextureFormat::Rgba32Uint,
            RawTextureFormat::Rgba8Sint => TextureFormat::Rgba8Sint,
            RawTextureFormat::Rgba8Snorm => TextureFormat::Rgba8Snorm,
            RawTextureFormat::Rgba8Uint => TextureFormat::Rgba8Uint,
            RawTextureFormat::Rgba8Unorm => TextureFormat::Rgba8Unorm,
            RawTextureFormat::Rgba8UnormSrgb => TextureFormat::Rgba8UnormSrgb,
            RawTextureFormat::Stencil8 => TextureFormat::Stencil8,
        }
    }
}
