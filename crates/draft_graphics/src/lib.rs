mod texture_format;

pub use texture_format::*;

use bitflags::bitflags;
use fyrox_core::{reflect::*, visitor::*};

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
