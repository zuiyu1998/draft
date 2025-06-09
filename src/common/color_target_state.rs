use fyrox_core::{reflect::*, visitor::*};

use crate::TextureFormat;

/// Color write mask. Disabled color channels will not be written to.
///
/// Corresponds to [WebGPU `GPUColorWriteFlags`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-gpucolorwriteflags).
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, Visit, Default)]
pub struct ColorWrites(u32);

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
