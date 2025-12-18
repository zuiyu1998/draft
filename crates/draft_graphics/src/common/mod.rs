use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};
use wgpu::{
    BlendComponent as WgpuBlendComponent, BlendFactor as WgpuBlendFactor,
    BlendOperation as WgpuBlendOperation, BlendState as WgpuBlendState, BufferAddress,
    ColorTargetState as WgpuColorTargetState, ColorWrites as WgpuColorWrites,
    CompareFunction as WgpuCompareFunction, DepthBiasState as WgpuDepthBiasState,
    DepthStencilState as WgpuDepthStencilState, Face as WgpuFace, FrontFace as WgpuFrontFace,
    IndexFormat as WgpuIndexFormat, MultisampleState as WgpuMultisampleState,
    PolygonMode as WgpuPolygonMode, PrimitiveState as WgpuPrimitiveState,
    PrimitiveTopology as WgpuPrimitiveTopology, ShaderLocation,
    StencilFaceState as WgpuStencilFaceState, StencilOperation as WgpuStencilOperation,
    StencilState as WgpuStencilState, TextureFormat as WgpuTextureFormat,
    VertexAttribute as WgpuVertexAttribute, VertexFormat as WgpuVertexFormat,
    VertexStepMode as WgpuVertexStepMode,
};

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum PolygonMode {
    /// Polygons are filled
    #[default]
    Fill = 0,
    /// Polygons are drawn as line segments
    Line = 1,
    /// Polygons are drawn as points
    Point = 2,
}

impl PolygonMode {
    pub fn get_wgpu_polygon_mode(&self) -> WgpuPolygonMode {
        match self {
            PolygonMode::Fill => WgpuPolygonMode::Fill,
            PolygonMode::Line => WgpuPolygonMode::Line,
            PolygonMode::Point => WgpuPolygonMode::Point,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum Face {
    /// Front face
    #[default]
    Front = 0,
    /// Back face
    Back = 1,
}

impl Face {
    pub fn get_wgpu_face(&self) -> WgpuFace {
        match self {
            Face::Back => WgpuFace::Back,
            Face::Front => WgpuFace::Front,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum FrontFace {
    /// Triangles with vertices in counter clockwise order are considered the front face.
    ///
    /// This is the default with right handed coordinate spaces.
    #[default]
    Ccw = 0,
    /// Triangles with vertices in clockwise order are considered the front face.
    ///
    /// This is the default with left handed coordinate spaces.
    Cw = 1,
}

impl FrontFace {
    pub fn get_wgpu_front_face(&self) -> WgpuFrontFace {
        match self {
            FrontFace::Ccw => WgpuFrontFace::Ccw,
            FrontFace::Cw => WgpuFrontFace::Cw,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum IndexFormat {
    /// Indices are 16 bit unsigned integers.
    Uint16 = 0,
    /// Indices are 32 bit unsigned integers.
    #[default]
    Uint32 = 1,
}

impl IndexFormat {
    pub fn get_wgpu_index_format(&self) -> WgpuIndexFormat {
        match self {
            IndexFormat::Uint16 => WgpuIndexFormat::Uint16,
            IndexFormat::Uint32 => WgpuIndexFormat::Uint32,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum PrimitiveTopology {
    /// Vertex data is a list of points. Each vertex is a new point.
    PointList = 0,
    /// Vertex data is a list of lines. Each pair of vertices composes a new line.
    ///
    /// Vertices `0 1 2 3` create two lines `0 1` and `2 3`
    LineList = 1,
    /// Vertex data is a strip of lines. Each set of two adjacent vertices form a line.
    ///
    /// Vertices `0 1 2 3` create three lines `0 1`, `1 2`, and `2 3`.
    LineStrip = 2,
    /// Vertex data is a list of triangles. Each set of 3 vertices composes a new triangle.
    ///
    /// Vertices `0 1 2 3 4 5` create two triangles `0 1 2` and `3 4 5`
    #[default]
    TriangleList = 3,
    /// Vertex data is a triangle strip. Each set of three adjacent vertices form a triangle.
    ///
    /// Vertices `0 1 2 3 4 5` create four triangles `0 1 2`, `2 1 3`, `2 3 4`, and `4 3 5`
    TriangleStrip = 4,
}

impl PrimitiveTopology {
    pub fn get_wgpu_primitive_topology(&self) -> WgpuPrimitiveTopology {
        match self {
            PrimitiveTopology::PointList => WgpuPrimitiveTopology::PointList,
            PrimitiveTopology::LineList => WgpuPrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => WgpuPrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => WgpuPrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => WgpuPrimitiveTopology::TriangleStrip,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: Option<Face>,
    pub unclipped_depth: bool,
    pub polygon_mode: PolygonMode,
    pub conservative: bool,
}

impl PrimitiveState {
    pub fn get_wgpu_primitive_state(&self) -> WgpuPrimitiveState {
        WgpuPrimitiveState {
            topology: self.topology.get_wgpu_primitive_topology(),
            strip_index_format: self
                .strip_index_format
                .as_ref()
                .map(|strip_index_format| strip_index_format.get_wgpu_index_format()),
            front_face: self.front_face.get_wgpu_front_face(),
            cull_mode: self
                .cull_mode
                .as_ref()
                .map(|cull_mode| cull_mode.get_wgpu_face()),
            unclipped_depth: self.unclipped_depth,
            polygon_mode: self.polygon_mode.get_wgpu_polygon_mode(),
            conservative: self.conservative,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
pub struct MultisampleState {
    pub count: u32,
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

impl MultisampleState {
    pub fn get_wgpu_multisample_state(&self) -> WgpuMultisampleState {
        WgpuMultisampleState {
            count: self.count,
            mask: self.mask,
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct DepthBiasState {
    pub constant: i32,
    pub slope_scale: f32,
    pub clamp: f32,
}

impl DepthBiasState {
    pub fn get_wgpu_depth_bias_state(&self) -> WgpuDepthBiasState {
        WgpuDepthBiasState {
            constant: self.constant,
            slope_scale: self.slope_scale,
            clamp: self.clamp,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
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

impl StencilOperation {
    pub fn get_wgpu_stencil_operation(&self) -> WgpuStencilOperation {
        match self {
            StencilOperation::Keep => WgpuStencilOperation::Keep,
            StencilOperation::Zero => WgpuStencilOperation::Zero,
            StencilOperation::Replace => WgpuStencilOperation::Replace,
            StencilOperation::Invert => WgpuStencilOperation::Invert,
            StencilOperation::IncrementClamp => WgpuStencilOperation::IncrementClamp,
            StencilOperation::DecrementClamp => WgpuStencilOperation::DecrementClamp,
            StencilOperation::IncrementWrap => WgpuStencilOperation::IncrementWrap,
            StencilOperation::DecrementWrap => WgpuStencilOperation::DecrementWrap,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
pub struct StencilFaceState {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

impl StencilFaceState {
    pub fn get_wgpu_stencil_face_state(&self) -> WgpuStencilFaceState {
        WgpuStencilFaceState {
            compare: self.compare.get_wgpu_compare_function(),
            fail_op: self.fail_op.get_wgpu_stencil_operation(),
            depth_fail_op: self.depth_fail_op.get_wgpu_stencil_operation(),
            pass_op: self.pass_op.get_wgpu_stencil_operation(),
        }
    }
}

impl Default for StencilFaceState {
    fn default() -> Self {
        Self {
            compare: CompareFunction::Always,
            fail_op: StencilOperation::Keep,
            depth_fail_op: StencilOperation::Keep,
            pass_op: StencilOperation::Keep,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
    pub read_mask: u32,
    pub write_mask: u32,
}

impl StencilState {
    pub fn get_wgpu_stencil_state(&self) -> WgpuStencilState {
        WgpuStencilState {
            front: self.front.get_wgpu_stencil_face_state(),
            back: self.front.get_wgpu_stencil_face_state(),
            read_mask: self.read_mask,
            write_mask: self.write_mask,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum CompareFunction {
    /// Function never passes
    Never = 1,
    /// Function passes if new value less than existing value
    #[default]
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

impl CompareFunction {
    pub fn get_wgpu_compare_function(&self) -> WgpuCompareFunction {
        match self {
            CompareFunction::Never => WgpuCompareFunction::Never,
            CompareFunction::Less => WgpuCompareFunction::Less,
            CompareFunction::Equal => WgpuCompareFunction::Equal,
            CompareFunction::LessEqual => WgpuCompareFunction::LessEqual,
            CompareFunction::Greater => WgpuCompareFunction::Greater,
            CompareFunction::NotEqual => WgpuCompareFunction::NotEqual,
            CompareFunction::GreaterEqual => WgpuCompareFunction::GreaterEqual,
            CompareFunction::Always => WgpuCompareFunction::Always,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil: StencilState,
    pub bias: DepthBiasState,
}

impl DepthStencilState {
    pub fn get_wgpu_depth_stencil_state(&self) -> WgpuDepthStencilState {
        WgpuDepthStencilState {
            format: self.format.get_wgpu_texture_format(),
            depth_write_enabled: self.depth_write_enabled,
            depth_compare: self.depth_compare.get_wgpu_compare_function(),
            stencil: self.stencil.get_wgpu_stencil_state(),
            bias: self.bias.get_wgpu_depth_bias_state(),
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
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

impl Default for ColorWrites {
    fn default() -> Self {
        Self::ALL
    }
}

impl ColorWrites {
    pub fn get_wgpu_color_writes(&self) -> WgpuColorWrites {
        WgpuColorWrites::from_bits(self.0).unwrap()
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
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

impl BlendOperation {
    pub fn get_wgpu_blend_operation(&self) -> WgpuBlendOperation {
        match self {
            BlendOperation::Add => WgpuBlendOperation::Add,
            BlendOperation::Subtract => WgpuBlendOperation::Subtract,
            BlendOperation::ReverseSubtract => WgpuBlendOperation::ReverseSubtract,
            BlendOperation::Min => WgpuBlendOperation::Min,
            BlendOperation::Max => WgpuBlendOperation::Max,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
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

impl BlendFactor {
    pub fn get_wgpu_blend_factor(&self) -> WgpuBlendFactor {
        match self {
            BlendFactor::Zero => WgpuBlendFactor::Zero,
            BlendFactor::One => WgpuBlendFactor::One,
            BlendFactor::Src => WgpuBlendFactor::Src,
            BlendFactor::OneMinusSrc => WgpuBlendFactor::OneMinusSrc,
            BlendFactor::SrcAlpha => WgpuBlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => WgpuBlendFactor::OneMinusSrcAlpha,
            BlendFactor::Dst => WgpuBlendFactor::Dst,
            BlendFactor::OneMinusDst => WgpuBlendFactor::OneMinusDst,
            BlendFactor::DstAlpha => WgpuBlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha => WgpuBlendFactor::OneMinusDstAlpha,
            BlendFactor::SrcAlphaSaturated => WgpuBlendFactor::SrcAlphaSaturated,
            BlendFactor::Constant => WgpuBlendFactor::Constant,
            BlendFactor::OneMinusConstant => WgpuBlendFactor::OneMinusConstant,
            BlendFactor::Src1 => WgpuBlendFactor::Src1,
            BlendFactor::OneMinusSrc1 => WgpuBlendFactor::OneMinusSrc1,
            BlendFactor::Src1Alpha => WgpuBlendFactor::Src1Alpha,
            BlendFactor::OneMinusSrc1Alpha => WgpuBlendFactor::OneMinusSrc1Alpha,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
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
    pub fn get_wgpu_blend_component(&self) -> WgpuBlendComponent {
        WgpuBlendComponent {
            src_factor: self.src_factor.get_wgpu_blend_factor(),
            dst_factor: self.dst_factor.get_wgpu_blend_factor(),
            operation: self.operation.get_wgpu_blend_operation(),
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct BlendState {
    /// Color equation.
    pub color: BlendComponent,
    /// Alpha equation.
    pub alpha: BlendComponent,
}

impl BlendState {
    pub fn get_blend_state(&self) -> WgpuBlendState {
        WgpuBlendState {
            color: self.color.get_wgpu_blend_component(),
            alpha: self.alpha.get_wgpu_blend_component(),
        }
    }
}

//todo impl more TextureFormat
#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub enum TextureFormat {
    #[default]
    R8Unorm,
    Rgba8UnormSrgb
}

impl TextureFormat {
    pub fn get_wgpu_texture_format(&self) -> WgpuTextureFormat {
        match self {
            TextureFormat::R8Unorm => WgpuTextureFormat::R8Unorm,
            TextureFormat::Rgba8UnormSrgb => WgpuTextureFormat::Rgba8UnormSrgb,
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub struct ColorTargetState {
    pub format: TextureFormat,
    pub blend: Option<BlendState>,
    pub write_mask: ColorWrites,
}

impl ColorTargetState {
    pub fn get_color_target_state(&self) -> WgpuColorTargetState {
        WgpuColorTargetState {
            format: self.format.get_wgpu_texture_format(),
            blend: self.blend.as_ref().map(|blend| blend.get_blend_state()),
            write_mask: self.write_mask.get_wgpu_color_writes(),
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default, Hash, PartialEq, Eq)]
pub struct VertexAttribute {
    pub format: VertexFormat,
    /// Byte offset of the start of the input
    pub offset: BufferAddress,
    /// Location for this input. Must match the location in the shader.
    pub shader_location: ShaderLocation,
}

impl VertexAttribute {
    pub fn get_wgpu_vertex_attribute(&self) -> WgpuVertexAttribute {
        WgpuVertexAttribute {
            format: self.format.get_wgpu_vertex_format(),
            offset: self.offset,
            shader_location: self.shader_location,
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default, Hash, PartialEq, Eq, Copy)]
pub enum VertexStepMode {
    /// Vertex data is advanced every vertex.
    #[default]
    Vertex = 0,
    /// Vertex data is advanced every instance.
    Instance = 1,
}

impl VertexStepMode {
    pub fn get_wgu_vertex_step_mode(&self) -> WgpuVertexStepMode {
        match self {
            VertexStepMode::Instance => WgpuVertexStepMode::Instance,
            VertexStepMode::Vertex => WgpuVertexStepMode::Vertex,
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default, Hash, PartialEq, Eq)]
pub struct VertexBufferLayout {
    pub array_stride: BufferAddress,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

//todo more VertexFormat
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Visit, Reflect, Default)]
pub enum VertexFormat {
    #[default]
    Uint8 = 0,
    /// Two unsigned bytes (u8). `vec2<u32>` in shaders.
    Uint8x2 = 1,
    /// Four unsigned bytes (u8). `vec4<u32>` in shaders.
    Uint8x4 = 2,
    /// One signed byte (i8). `i32` in shaders.
    Sint8 = 3,
    /// Two signed bytes (i8). `vec2<i32>` in shaders.
    Sint8x2 = 4,
    /// Four signed bytes (i8). `vec4<i32>` in shaders.
    Sint8x4 = 5,
    /// One unsigned byte (u8). [0, 255] converted to float [0, 1] `f32` in shaders.
    Unorm8 = 6,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm8x2 = 7,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm8x4 = 8,
    /// One signed byte (i8). [&minus;127, 127] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm8 = 9,
    /// Two signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm8x2 = 10,
    /// Four signed bytes (i8). [&minus;127, 127] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
    Snorm8x4 = 11,
    /// One unsigned short (u16). `u32` in shaders.
    Uint16 = 12,
    /// Two unsigned shorts (u16). `vec2<u32>` in shaders.
    Uint16x2 = 13,
    /// Four unsigned shorts (u16). `vec4<u32>` in shaders.
    Uint16x4 = 14,
    /// One signed short (u16). `i32` in shaders.
    Sint16 = 15,
    /// Two signed shorts (i16). `vec2<i32>` in shaders.
    Sint16x2 = 16,
    /// Four signed shorts (i16). `vec4<i32>` in shaders.
    Sint16x4 = 17,
    /// One unsigned short (u16). [0, 65535] converted to float [0, 1] `f32` in shaders.
    Unorm16 = 18,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2<f32>` in shaders.
    Unorm16x2 = 19,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4<f32>` in shaders.
    Unorm16x4 = 20,
    /// One signed short (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `f32` in shaders.
    Snorm16 = 21,
    /// Two signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec2<f32>` in shaders.
    Snorm16x2 = 22,
    /// Four signed shorts (i16). [&minus;32767, 32767] converted to float [&minus;1, 1] `vec4<f32>` in shaders.
    Snorm16x4 = 23,
    /// One half-precision float (no Rust equiv). `f32` in shaders.
    Float16 = 24,
    /// Two half-precision floats (no Rust equiv). `vec2<f32>` in shaders.
    Float16x2 = 25,
    /// Four half-precision floats (no Rust equiv). `vec4<f32>` in shaders.
    Float16x4 = 26,
    /// One single-precision float (f32). `f32` in shaders.
    Float32 = 27,
    /// Two single-precision floats (f32). `vec2<f32>` in shaders.
    Float32x2 = 28,
    /// Three single-precision floats (f32). `vec3<f32>` in shaders.
    Float32x3 = 29,
    /// Four single-precision floats (f32). `vec4<f32>` in shaders.
    Float32x4 = 30,
    /// One unsigned int (u32). `u32` in shaders.
    Uint32 = 31,
    /// Two unsigned ints (u32). `vec2<u32>` in shaders.
    Uint32x2 = 32,
    /// Three unsigned ints (u32). `vec3<u32>` in shaders.
    Uint32x3 = 33,
    /// Four unsigned ints (u32). `vec4<u32>` in shaders.
    Uint32x4 = 34,
    /// One signed int (i32). `i32` in shaders.
    Sint32 = 35,
    /// Two signed ints (i32). `vec2<i32>` in shaders.
    Sint32x2 = 36,
    /// Three signed ints (i32). `vec3<i32>` in shaders.
    Sint32x3 = 37,
    /// Four signed ints (i32). `vec4<i32>` in shaders.
    Sint32x4 = 38,
    /// One double-precision float (f64). `f32` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64 = 39,
    /// Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x2 = 40,
    /// Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x3 = 41,
    /// Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
    Float64x4 = 42,
}

impl VertexFormat {
    /// Returns the byte size of the format.
    #[must_use]
    pub const fn size(&self) -> u64 {
        match self {
            Self::Uint8 | Self::Sint8 | Self::Unorm8 | Self::Snorm8 => 1,
            Self::Uint8x2
            | Self::Sint8x2
            | Self::Unorm8x2
            | Self::Snorm8x2
            | Self::Uint16
            | Self::Sint16
            | Self::Unorm16
            | Self::Snorm16
            | Self::Float16 => 2,
            Self::Uint8x4
            | Self::Sint8x4
            | Self::Unorm8x4
            | Self::Snorm8x4
            | Self::Uint16x2
            | Self::Sint16x2
            | Self::Unorm16x2
            | Self::Snorm16x2
            | Self::Float16x2
            | Self::Float32
            | Self::Uint32
            | Self::Sint32 => 4,
            Self::Uint16x4
            | Self::Sint16x4
            | Self::Unorm16x4
            | Self::Snorm16x4
            | Self::Float16x4
            | Self::Float32x2
            | Self::Uint32x2
            | Self::Sint32x2
            | Self::Float64 => 8,
            Self::Float32x3 | Self::Uint32x3 | Self::Sint32x3 => 12,
            Self::Float32x4 | Self::Uint32x4 | Self::Sint32x4 | Self::Float64x2 => 16,
            Self::Float64x3 => 24,
            Self::Float64x4 => 32,
        }
    }

    pub fn get_wgpu_vertex_format(&self) -> WgpuVertexFormat {
        match self {
            VertexFormat::Uint8 => WgpuVertexFormat::Uint8,
            VertexFormat::Uint8x2 => WgpuVertexFormat::Uint8x2,
            VertexFormat::Uint8x4 => WgpuVertexFormat::Uint8x4,
            VertexFormat::Sint8 => WgpuVertexFormat::Sint8,
            VertexFormat::Sint8x2 => WgpuVertexFormat::Sint8x2,
            VertexFormat::Sint8x4 => WgpuVertexFormat::Sint8x4,
            VertexFormat::Unorm8 => WgpuVertexFormat::Unorm8,
            VertexFormat::Unorm8x2 => WgpuVertexFormat::Unorm8x2,
            VertexFormat::Unorm8x4 => WgpuVertexFormat::Unorm8x4,
            VertexFormat::Snorm8 => WgpuVertexFormat::Snorm8,
            VertexFormat::Snorm8x2 => WgpuVertexFormat::Snorm8x2,
            VertexFormat::Snorm8x4 => WgpuVertexFormat::Snorm8x4,
            VertexFormat::Uint16 => WgpuVertexFormat::Uint16,
            VertexFormat::Uint16x2 => WgpuVertexFormat::Uint16x2,
            VertexFormat::Uint16x4 => WgpuVertexFormat::Uint16x4,
            VertexFormat::Sint16 => WgpuVertexFormat::Sint16,
            VertexFormat::Sint16x2 => WgpuVertexFormat::Sint16x2,
            VertexFormat::Sint16x4 => WgpuVertexFormat::Sint16x4,
            VertexFormat::Unorm16 => WgpuVertexFormat::Unorm16,
            VertexFormat::Unorm16x2 => WgpuVertexFormat::Unorm16x2,
            VertexFormat::Unorm16x4 => WgpuVertexFormat::Unorm16x4,
            VertexFormat::Snorm16 => WgpuVertexFormat::Snorm16,
            VertexFormat::Snorm16x2 => WgpuVertexFormat::Snorm16x2,
            VertexFormat::Snorm16x4 => WgpuVertexFormat::Snorm16x4,
            VertexFormat::Float16 => WgpuVertexFormat::Float16,
            VertexFormat::Float16x2 => WgpuVertexFormat::Float16x2,
            VertexFormat::Float16x4 => WgpuVertexFormat::Float16x4,
            VertexFormat::Float32 => WgpuVertexFormat::Float32,
            VertexFormat::Float32x2 => WgpuVertexFormat::Float32x2,
            VertexFormat::Float32x3 => WgpuVertexFormat::Float32x3,
            VertexFormat::Float32x4 => WgpuVertexFormat::Float32x4,
            VertexFormat::Uint32 => WgpuVertexFormat::Uint32,
            VertexFormat::Uint32x2 => WgpuVertexFormat::Uint32x2,
            VertexFormat::Uint32x3 => WgpuVertexFormat::Uint32x3,
            VertexFormat::Uint32x4 => WgpuVertexFormat::Uint32x4,
            VertexFormat::Sint32 => WgpuVertexFormat::Sint32,
            VertexFormat::Sint32x2 => WgpuVertexFormat::Sint32x2,
            VertexFormat::Sint32x3 => WgpuVertexFormat::Sint32x3,
            VertexFormat::Sint32x4 => WgpuVertexFormat::Sint32x4,
            VertexFormat::Float64 => WgpuVertexFormat::Float64,
            VertexFormat::Float64x2 => WgpuVertexFormat::Float64x2,
            VertexFormat::Float64x3 => WgpuVertexFormat::Float64x3,
            VertexFormat::Float64x4 => WgpuVertexFormat::Float64x4,
        }
    }
}
