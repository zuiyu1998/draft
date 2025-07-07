use std::{
    collections::HashMap,
    num::{NonZeroU32, NonZeroU64},
};

use crate::gfx_base::{
    RawBindGroupLayoutEntry, RawBindingType, RawBufferBindingType, RawPipelineCompilationOptions,
    RawSamplerBindingType, RawShaderStages, RawStorageTextureAccess, RawTextureSampleType,
    RawTextureViewDimension, TextureFormat,
};

use fyrox_core::{reflect::*, visitor::*};

#[derive(Clone, Debug, Visit, Reflect, Copy, PartialEq, Eq, Hash)]
pub struct ShaderStages(u32);

impl Default for ShaderStages {
    fn default() -> Self {
        ShaderStages::VERTEX_FRAGMENT
    }
}

impl From<ShaderStages> for RawShaderStages {
    fn from(value: ShaderStages) -> Self {
        RawShaderStages::from_bits(value.0).unwrap()
    }
}

bitflags::bitflags! {
    impl ShaderStages: u32 {
           /// Binding is not visible from any shader stage.
        const NONE = 0;
        /// Binding is visible from the vertex shader of a render pipeline.
        const VERTEX = 1 << 0;
        /// Binding is visible from the fragment shader of a render pipeline.
        const FRAGMENT = 1 << 1;
        /// Binding is visible from the compute shader of a compute pipeline.
        const COMPUTE = 1 << 2;
        /// Binding is visible from the vertex and fragment shaders of a render pipeline.
        const VERTEX_FRAGMENT = Self::VERTEX.bits() | Self::FRAGMENT.bits();
    }
}

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

impl From<RawTextureViewDimension> for TextureViewDimension {
    fn from(value: RawTextureViewDimension) -> Self {
        match value {
            RawTextureViewDimension::Cube => TextureViewDimension::Cube,
            RawTextureViewDimension::D1 => TextureViewDimension::D1,
            RawTextureViewDimension::CubeArray => TextureViewDimension::CubeArray,
            RawTextureViewDimension::D2 => TextureViewDimension::D2,
            RawTextureViewDimension::D2Array => TextureViewDimension::D2Array,
            RawTextureViewDimension::D3 => TextureViewDimension::D3,
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
    pub visibility: ShaderStages,
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
            visibility: value.visibility.into(),
            ty: value.ty.into(),
            count: value
                .count
                .and_then(|value| NonZeroU32::try_from(value).ok()),
        }
    }
}
