use fyrox_core::{reflect::*, visitor::*};

use frame_graph::wgpu::MultisampleState as RawMultisampleState;

/// Describes the multi-sampling state of a render pipeline.
///
/// Corresponds to [WebGPU `GPUMultisampleState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpumultisamplestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Visit)]
pub struct MultisampleState {
    /// The number of samples calculated per pixel (for MSAA). For non-multisampled textures,
    /// this should be `1`
    pub count: u32,
    /// Bitmask that restricts the samples of a pixel modified by this pipeline. All samples
    /// can be enabled using the value `!0`
    pub mask: u64,
    pub alpha_to_coverage_enabled: bool,
}

impl Default for MultisampleState {
    fn default() -> Self {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }
}

impl From<MultisampleState> for RawMultisampleState {
    fn from(value: MultisampleState) -> Self {
        RawMultisampleState {
            count: value.count,
            mask: value.mask,
            alpha_to_coverage_enabled: value.alpha_to_coverage_enabled,
        }
    }
}
