mod texture_format;

pub use texture_format::*;

use bitflags::bitflags;
use fyrox_core::{reflect::*, visitor::*};

#[derive(Debug, Clone, Visit, Reflect)]
pub struct SamplerDescriptor {
    pub label: Option<String>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<SamplerBorderColor>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            label: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default)]

pub enum SamplerBorderColor {
    /// [0, 0, 0, 0]
    #[default]
    TransparentBlack,
    /// [0, 0, 0, 1]
    OpaqueBlack,
    /// [1, 1, 1, 1]
    OpaqueWhite,

    /// On the Metal backend, this is equivalent to `TransparentBlack` for
    /// textures that have an alpha component, and equivalent to `OpaqueBlack`
    /// for textures that do not have an alpha component. On other backends,
    /// this is equivalent to `TransparentBlack`. Requires
    /// [`Features::ADDRESS_MODE_CLAMP_TO_ZERO`]. Not supported on the web.
    Zero,
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub enum CompareFunction {
    /// Function never passes
    Never = 1,
    /// Function passes if new value less than existing value
    Less = 2,
    /// Function passes if new value is equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    Equal = 3,
    /// Function passes if new value is less than or equal to existing value
    LessEqual = 4,
    /// Function passes if new value is greater than existing value
    Greater = 5,
    /// Function passes if new value is not equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    NotEqual = 6,
    /// Function passes if new value is greater than or equal to existing value
    GreaterEqual = 7,
    /// Function always passes
    #[default]
    Always = 8,
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub enum MipmapFilterMode {
    /// Nearest neighbor sampling.
    ///
    /// Return the value of the texel nearest to the texture coordinates.
    #[default]
    Nearest = 0,
    /// Linear Interpolation
    ///
    /// Select two texels in each dimension and return a linear interpolation between their values.
    Linear = 1,
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub enum FilterMode {
    /// Nearest neighbor sampling.
    ///
    /// This creates a pixelated effect.
    #[default]
    Nearest = 0,
    /// Linear Interpolation
    ///
    /// This makes textures smooth but blurry.
    Linear = 1,
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub enum AddressMode {
    /// Clamp the value to the edge of the texture
    ///
    /// -0.25 -> 0.0
    /// 1.25  -> 1.0
    #[default]
    ClampToEdge = 0,
    /// Repeat the texture in a tiling fashion
    ///
    /// -0.25 -> 0.75
    /// 1.25 -> 0.25
    Repeat = 1,
    /// Repeat the texture, mirroring it every repeat
    ///
    /// -0.25 -> 0.25
    /// 1.25 -> 0.75
    MirrorRepeat = 2,
    /// Clamp the value to the border of the texture
    /// Requires feature [`Features::ADDRESS_MODE_CLAMP_TO_BORDER`]
    ///
    /// -0.25 -> border
    /// 1.25 -> border
    ClampToBorder = 3,
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub enum TextureAspect {
    /// Depth, Stencil, and Color.
    #[default]
    All,
    /// Stencil.
    StencilOnly,
    /// Depth.
    DepthOnly,
    /// Plane 0.
    Plane0,
    /// Plane 1.
    Plane1,
    /// Plane 2.
    Plane2,
}

#[derive(Debug, Clone, Visit, Reflect)]
pub struct TextureUsages(u32);

bitflags! {
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
        ///
        /// Consider adding [`TextureUsages::TRANSIENT`] if the contents are not reused.
        const RENDER_ATTACHMENT = 1 << 4;

        //
        // ---- Restart Numbering for Native Features ---
        //
        // Native Features:
        //
        /// Allows a texture to be used with image atomics. Requires [`Features::TEXTURE_ATOMIC`].
        const STORAGE_ATOMIC = 1 << 16;
        /// Specifies the contents of this texture will not be used in another pass to potentially reduce memory usage and bandwidth.
        ///
        /// No-op on platforms on platforms that do not benefit from transient textures.
        /// Generally mobile and Apple chips care about this.
        ///
        /// Incompatible with ALL other usages except [`TextureUsages::RENDER_ATTACHMENT`] and requires it.
        ///
        /// Requires [`StoreOp::Discard`].
        const TRANSIENT = 1 << 17;
    }
}

#[derive(Debug, Clone, Visit, Reflect)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
}

#[derive(Debug, Clone, Visit, Reflect, Copy)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

#[derive(Debug, Clone, Visit, Reflect)]
pub struct TextureDescriptor {
    pub label: Option<String>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: Vec<TextureFormat>,
}
