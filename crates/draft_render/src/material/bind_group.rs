use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, Visit, Reflect)]
pub struct ShaderStages(u32);

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
        /// Binding is visible from the task shader of a mesh pipeline
        const TASK = 1 << 3;
        /// Binding is visible from the mesh shader of a mesh pipeline
        const MESH = 1 << 4;
     }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum BufferBindingType {
    #[default]
    Uniform,
    Storage {
        read_only: bool,
    },
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
pub enum TextureSampleType {
    Float { filterable: bool },
    Depth,
    Sint,
    Uint,
}

impl Default for TextureSampleType {
    fn default() -> Self {
        TextureSampleType::Float { filterable: false }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum TextureViewDimension {
    D1,
    #[default]
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum StorageTextureAccess {
    #[default]
    WriteOnly,
    ReadOnly,
    Atomic,
}

//todo impl more TextureFormat
#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum TextureFormat {
    #[default]
    R8Unorm,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum BindingType {
    #[default]
    ExternalTexture,
    Buffer {
        ty: BufferBindingType,
        has_dynamic_offset: bool,
        min_binding_size: u64,
    },
    Texture {
        sample_type: TextureSampleType,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: TextureFormat,
        view_dimension: TextureViewDimension,
    },
    AccelerationStructure {
        vertex_return: bool,
    },
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: ShaderStages,
    pub ty: BindingType,
    pub count: u32,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroupLayout {
    pub name: String,
    pub entries: Vec<MaterialBindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialBindGroup {
    name: String,
    layouts: Vec<MaterialBindGroupLayout>,
}
