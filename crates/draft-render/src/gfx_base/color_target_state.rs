use fyrox_core::{reflect::*, visitor::*};

use crate::gfx_base::{
    RawBlendComponent, RawBlendFactor, RawBlendOperation, RawBlendState, RawColorTargetState,
    RawColorWrites, TextureFormat,
};

/// Color write mask. Disabled color channels will not be written to.
///
/// Corresponds to [WebGPU `GPUColorWriteFlags`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-gpucolorwriteflags).
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, Visit, Default)]
pub struct ColorWrites(u32);

bitflags::bitflags! {
    impl ColorWrites: u32 {
        /// Enable red channel writes
        const RED = 1 << 0;
        /// Enable green channel writes
        const GREEN = 1 << 1;
        /// Enable blue channel writes
        const BLUE = 1 << 2;
        /// Enable alpha channel writes
        const ALPHA = 1 << 3;
        /// Enable red, green, and blue channel writes
        const COLOR = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
        /// Enable writes to all channels.
        const ALL = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits() | Self::ALPHA.bits();
    }
}
impl From<ColorWrites> for RawColorWrites {
    fn from(value: ColorWrites) -> Self {
        RawColorWrites::from_bits(value.0).unwrap_or(RawColorWrites::ALL)
    }
}

/// Alpha blend operation.
///
/// Corresponds to [WebGPU `GPUBlendOperation`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpublendoperation).
///
/// For further details on how the blend operations are applied, see
/// the analogous functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Blending#Blend_Equations>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Reflect, Visit)]
pub enum BlendOperation {
    /// Src + Dst
    #[default]
    Add = 0,
    /// Src - Dst
    Subtract = 1,
    /// Dst - Src
    ReverseSubtract = 2,
    /// min(Src, Dst)
    Min = 3,
    /// max(Src, Dst)
    Max = 4,
}

impl From<BlendOperation> for RawBlendOperation {
    fn from(value: BlendOperation) -> Self {
        match value {
            BlendOperation::Add => RawBlendOperation::Add,
            BlendOperation::Subtract => RawBlendOperation::Subtract,
            BlendOperation::ReverseSubtract => RawBlendOperation::ReverseSubtract,
            BlendOperation::Min => RawBlendOperation::Min,
            BlendOperation::Max => RawBlendOperation::Max,
        }
    }
}

/// Alpha blend factor.
///
/// Corresponds to [WebGPU `GPUBlendFactor`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpublendfactor). Values using `Src1`
/// require [`Features::DUAL_SOURCE_BLENDING`] and can only be used with the first
/// render target.
///
/// For further details on how the blend factors are applied, see the analogous
/// functionality in OpenGL: <https://www.khronos.org/opengl/wiki/Blending#Blending_Parameters>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Reflect, Visit, Default)]
pub enum BlendFactor {
    /// 0.0
    #[default]
    Zero = 0,
    /// 1.0
    One = 1,
    /// S.component
    Src = 2,
    /// 1.0 - S.component
    OneMinusSrc = 3,
    /// S.alpha
    SrcAlpha = 4,
    /// 1.0 - S.alpha
    OneMinusSrcAlpha = 5,
    /// D.component
    Dst = 6,
    /// 1.0 - D.component
    OneMinusDst = 7,
    /// D.alpha
    DstAlpha = 8,
    /// 1.0 - D.alpha
    OneMinusDstAlpha = 9,
    /// min(S.alpha, 1.0 - D.alpha)
    SrcAlphaSaturated = 10,
    /// Constant
    Constant = 11,
    /// 1.0 - Constant
    OneMinusConstant = 12,
    /// S1.component
    Src1 = 13,
    /// 1.0 - S1.component
    OneMinusSrc1 = 14,
    /// S1.alpha
    Src1Alpha = 15,
    /// 1.0 - S1.alpha
    OneMinusSrc1Alpha = 16,
}

impl From<BlendFactor> for RawBlendFactor {
    fn from(value: BlendFactor) -> Self {
        match value {
            BlendFactor::Constant => RawBlendFactor::Constant,
            BlendFactor::Dst => RawBlendFactor::Dst,
            BlendFactor::DstAlpha => RawBlendFactor::DstAlpha,
            BlendFactor::One => RawBlendFactor::One,
            BlendFactor::OneMinusConstant => RawBlendFactor::OneMinusConstant,
            BlendFactor::OneMinusDst => RawBlendFactor::OneMinusDst,
            BlendFactor::OneMinusDstAlpha => RawBlendFactor::OneMinusDstAlpha,
            BlendFactor::OneMinusSrc => RawBlendFactor::OneMinusSrc,
            BlendFactor::OneMinusSrc1 => RawBlendFactor::OneMinusSrc1,
            BlendFactor::OneMinusSrc1Alpha => RawBlendFactor::OneMinusSrc1Alpha,
            BlendFactor::OneMinusSrcAlpha => RawBlendFactor::OneMinusSrcAlpha,
            BlendFactor::SrcAlphaSaturated => RawBlendFactor::SrcAlphaSaturated,
            BlendFactor::Zero => RawBlendFactor::Zero,
            BlendFactor::Src => RawBlendFactor::Src,
            BlendFactor::Src1Alpha => RawBlendFactor::Src1Alpha,
            BlendFactor::SrcAlpha => RawBlendFactor::SrcAlpha,
            BlendFactor::Src1 => RawBlendFactor::Src1,
        }
    }
}

/// Describes a blend component of a [`BlendState`].
///
/// Corresponds to [WebGPU `GPUBlendComponent`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpublendcomponent).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Visit, Default)]
pub struct BlendComponent {
    /// Multiplier for the source, which is produced by the fragment shader.
    pub src_factor: BlendFactor,
    /// Multiplier for the destination, which is stored in the target.
    pub dst_factor: BlendFactor,
    /// The binary operation applied to the source and destination,
    /// multiplied by their respective factors.
    pub operation: BlendOperation,
}

impl BlendComponent {
    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };
}

impl From<BlendComponent> for RawBlendComponent {
    fn from(value: BlendComponent) -> Self {
        RawBlendComponent {
            src_factor: value.src_factor.into(),
            dst_factor: value.dst_factor.into(),
            operation: value.operation.into(),
        }
    }
}

/// Describe the blend state of a render pipeline,
/// within [`ColorTargetState`].
///
/// Corresponds to [WebGPU `GPUBlendState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpublendstate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Default, Visit)]
pub struct BlendState {
    /// Color equation.
    pub color: BlendComponent,
    /// Alpha equation.
    pub alpha: BlendComponent,
}

impl From<BlendState> for RawBlendState {
    fn from(value: BlendState) -> Self {
        RawBlendState {
            color: value.color.into(),
            alpha: value.alpha.into(),
        }
    }
}

/// Describes the color state of a render pipeline.
///
/// Corresponds to [WebGPU `GPUColorTargetState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucolortargetstate).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, Visit, Default)]
pub struct ColorTargetState {
    /// The [`TextureFormat`] of the image that this pipeline will render to. Must match the format
    /// of the corresponding color attachment in [`CommandEncoder::begin_render_pass`][CEbrp]
    ///
    /// [CEbrp]: ../wgpu/struct.CommandEncoder.html#method.begin_render_pass
    pub format: TextureFormat,
    /// The blending that is used for this pipeline.
    pub blend: Option<BlendState>,
    /// Mask which enables/disables writes to different color/alpha channel.
    pub write_mask: ColorWrites,
}

impl<'a> From<&'a ColorTargetState> for RawColorTargetState {
    fn from(value: &'a ColorTargetState) -> Self {
        RawColorTargetState {
            format: value.format.into(),
            blend: value.blend.map(Into::into),
            write_mask: value.write_mask.into(),
        }
    }
}
