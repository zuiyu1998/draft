use std::hash::{Hash, Hasher};

use crate::TextureFormat;
use fyrox_core::{reflect::*, visitor::*};
use wgpu::{
    CompareFunction as RawCompareFunction, DepthBiasState as RawDepthBiasState,
    DepthStencilState as RawDepthStencilState, StencilFaceState as RawStencilFaceState,
    StencilOperation as RawStencilOperation, StencilState as RawStencilState,
};

/// Describes the biasing setting for the depth target.
///
/// For use in [`DepthStencilState`].
///
/// Corresponds to a portion of [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Reflect, Visit)]
pub struct DepthBiasState {
    /// Constant depth biasing factor, in basic units of the depth format.
    pub constant: i32,
    /// Slope depth biasing factor.
    pub slope_scale: f32,
    /// Depth bias clamp value (absolute).
    pub clamp: f32,
}

impl From<DepthBiasState> for RawDepthBiasState {
    fn from(value: DepthBiasState) -> Self {
        Self {
            constant: value.constant,
            slope_scale: value.slope_scale,
            clamp: value.clamp,
        }
    }
}

impl Hash for DepthBiasState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.constant.hash(state);
        self.slope_scale.to_bits().hash(state);
        self.clamp.to_bits().hash(state);
    }
}

impl PartialEq for DepthBiasState {
    fn eq(&self, other: &Self) -> bool {
        (self.constant == other.constant)
            && (self.slope_scale.to_bits() == other.slope_scale.to_bits())
            && (self.clamp.to_bits() == other.clamp.to_bits())
    }
}

impl Eq for DepthBiasState {}

/// Operation to perform on the stencil value.
///
/// Corresponds to [WebGPU `GPUStencilOperation`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpustenciloperation).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Visit, Reflect)]
pub enum StencilOperation {
    /// Keep stencil value unchanged.
    #[default]
    Keep = 0,
    /// Set stencil value to zero.
    Zero = 1,
    /// Replace stencil value with value provided in most recent call to
    /// [`RenderPass::set_stencil_reference`][RPssr].
    ///
    /// [RPssr]: ../wgpu/struct.RenderPass.html#method.set_stencil_reference
    Replace = 2,
    /// Bitwise inverts stencil value.
    Invert = 3,
    /// Increments stencil value by one, clamping on overflow.
    IncrementClamp = 4,
    /// Decrements stencil value by one, clamping on underflow.
    DecrementClamp = 5,
    /// Increments stencil value by one, wrapping on overflow.
    IncrementWrap = 6,
    /// Decrements stencil value by one, wrapping on underflow.
    DecrementWrap = 7,
}

impl From<StencilOperation> for RawStencilOperation {
    fn from(value: StencilOperation) -> Self {
        match value {
            StencilOperation::Keep => RawStencilOperation::Keep,
            StencilOperation::Zero => RawStencilOperation::Zero,
            StencilOperation::Replace => RawStencilOperation::Replace,
            StencilOperation::Invert => RawStencilOperation::Invert,
            StencilOperation::IncrementClamp => RawStencilOperation::IncrementClamp,
            StencilOperation::DecrementClamp => RawStencilOperation::DecrementClamp,
            StencilOperation::IncrementWrap => RawStencilOperation::IncrementWrap,
            StencilOperation::DecrementWrap => RawStencilOperation::DecrementWrap,
        }
    }
}

/// Describes stencil state in a render pipeline.
///
/// If you are not using stencil state, set this to [`StencilFaceState::IGNORE`].
///
/// Corresponds to [WebGPU `GPUStencilFaceState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpustencilfacestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Default, Visit)]
pub struct StencilFaceState {
    /// Comparison function that determines if the fail_op or pass_op is used on the stencil buffer.
    pub compare: CompareFunction,
    /// Operation that is performed when stencil test fails.
    pub fail_op: StencilOperation,
    /// Operation that is performed when depth test fails but stencil test succeeds.
    pub depth_fail_op: StencilOperation,
    /// Operation that is performed when stencil test success.
    pub pass_op: StencilOperation,
}

impl From<StencilFaceState> for RawStencilFaceState {
    fn from(value: StencilFaceState) -> Self {
        RawStencilFaceState {
            compare: value.compare.into(),
            fail_op: value.fail_op.into(),
            depth_fail_op: value.depth_fail_op.into(),
            pass_op: value.pass_op.into(),
        }
    }
}

/// State of the stencil operation (fixed-pipeline stage).
///
/// For use in [`DepthStencilState`].
///
/// Corresponds to a portion of [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Reflect, Visit)]
pub struct StencilState {
    /// Front face mode.
    pub front: StencilFaceState,
    /// Back face mode.
    pub back: StencilFaceState,
    /// Stencil values are AND'd with this mask when reading and writing from the stencil buffer. Only low 8 bits are used.
    pub read_mask: u32,
    /// Stencil values are AND'd with this mask when writing to the stencil buffer. Only low 8 bits are used.
    pub write_mask: u32,
}

impl<'a> From<&'a StencilState> for RawStencilState {
    fn from(value: &'a StencilState) -> Self {
        RawStencilState {
            front: value.front.into(),
            back: value.back.into(),
            read_mask: value.read_mask,
            write_mask: value.write_mask,
        }
    }
}

/// Comparison function used for depth and stencil operations.
///
/// Corresponds to [WebGPU `GPUCompareFunction`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpucomparefunction).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Visit, Reflect, Default)]
pub enum CompareFunction {
    /// Function never passes
    #[default]
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
    Always = 8,
}

impl From<CompareFunction> for RawCompareFunction {
    fn from(value: CompareFunction) -> Self {
        match value {
            CompareFunction::Always => RawCompareFunction::Always,
            CompareFunction::Equal => RawCompareFunction::Equal,
            CompareFunction::Greater => RawCompareFunction::Greater,
            CompareFunction::GreaterEqual => RawCompareFunction::GreaterEqual,
            CompareFunction::Less => RawCompareFunction::Less,
            CompareFunction::LessEqual => RawCompareFunction::LessEqual,
            CompareFunction::Never => RawCompareFunction::Never,
            CompareFunction::NotEqual => RawCompareFunction::NotEqual,
        }
    }
}

/// Describes the depth/stencil state in a render pipeline.
///
/// Corresponds to [WebGPU `GPUDepthStencilState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpudepthstencilstate).
#[repr(C)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Reflect, Visit, Default)]
pub struct DepthStencilState {
    /// Format of the depth/stencil buffer, must be special depth format. Must match the format
    /// of the depth/stencil attachment in [`CommandEncoder::begin_render_pass`][CEbrp].
    ///
    /// [CEbrp]: ../wgpu/struct.CommandEncoder.html#method.begin_render_pass
    pub format: TextureFormat,
    /// If disabled, depth will not be written to.
    pub depth_write_enabled: bool,
    /// Comparison function used to compare depth values in the depth test.
    pub depth_compare: CompareFunction,
    /// Stencil state.
    pub stencil: StencilState,
    /// Depth bias state.
    pub bias: DepthBiasState,
}

impl<'a> From<&'a DepthStencilState> for RawDepthStencilState {
    fn from(value: &'a DepthStencilState) -> Self {
        RawDepthStencilState {
            format: value.format.into(),
            depth_write_enabled: value.depth_write_enabled,
            depth_compare: value.depth_compare.into(),
            stencil: (&value.stencil).into(),
            bias: value.bias.into(),
        }
    }
}
