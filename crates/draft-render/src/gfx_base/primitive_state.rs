use crate::gfx_base::{
    RawFace, RawFrontFace, RawIndexFormat, RawPolygonMode, RawPrimitiveState, RawPrimitiveTopology,
};
use fyrox_core::{reflect::*, visitor::*};

/// Type of drawing mode for polygons
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Reflect, Visit)]
pub enum PolygonMode {
    /// Polygons are filled
    #[default]
    Fill = 0,
    /// Polygons are drawn as line segments
    Line = 1,
    /// Polygons are drawn as points
    Point = 2,
}

impl From<PolygonMode> for RawPolygonMode {
    fn from(value: PolygonMode) -> Self {
        match value {
            PolygonMode::Fill => RawPolygonMode::Fill,
            PolygonMode::Line => RawPolygonMode::Line,
            PolygonMode::Point => RawPolygonMode::Point,
        }
    }
}

/// Face of a vertex.
///
/// Corresponds to [WebGPU `GPUCullMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpucullmode),
/// except that the `"none"` value is represented using `Option<Face>` instead.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Visit, Reflect, Default)]
pub enum Face {
    /// Front face
    #[default]
    Front = 0,
    /// Back face
    Back = 1,
}

impl From<Face> for RawFace {
    fn from(value: Face) -> Self {
        match value {
            Face::Back => RawFace::Back,
            Face::Front => RawFace::Front,
        }
    }
}

/// Vertex winding order which classifies the "front" face of a triangle.
///
/// Corresponds to [WebGPU `GPUFrontFace`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpufrontface).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Visit, Reflect)]
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

impl From<FrontFace> for RawFrontFace {
    fn from(value: FrontFace) -> Self {
        match value {
            FrontFace::Ccw => RawFrontFace::Ccw,
            FrontFace::Cw => RawFrontFace::Cw,
        }
    }
}

/// Format of indices used with pipeline.
///
/// Corresponds to [WebGPU `GPUIndexFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuindexformat).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Visit, Reflect)]
pub enum IndexFormat {
    /// Indices are 16 bit unsigned integers.
    Uint16 = 0,
    /// Indices are 32 bit unsigned integers.
    #[default]
    Uint32 = 1,
}

impl From<IndexFormat> for RawIndexFormat {
    fn from(value: IndexFormat) -> Self {
        match value {
            IndexFormat::Uint16 => RawIndexFormat::Uint16,
            IndexFormat::Uint32 => RawIndexFormat::Uint32,
        }
    }
}

/// Primitive type the input mesh is composed of.
///
/// Corresponds to [WebGPU `GPUPrimitiveTopology`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuprimitivetopology).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Visit, Reflect)]
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

impl From<PrimitiveTopology> for RawPrimitiveTopology {
    fn from(value: PrimitiveTopology) -> Self {
        match value {
            PrimitiveTopology::PointList => RawPrimitiveTopology::PointList,
            PrimitiveTopology::LineStrip => RawPrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => RawPrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => RawPrimitiveTopology::TriangleStrip,
            PrimitiveTopology::LineList => RawPrimitiveTopology::LineList,
        }
    }
}

/// Describes the state of primitive assembly and rasterization in a render pipeline.
///
/// Corresponds to [WebGPU `GPUPrimitiveState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuprimitivestate).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect, Visit)]
pub struct PrimitiveState {
    /// The primitive topology used to interpret vertices.
    pub topology: PrimitiveTopology,
    /// When drawing strip topologies with indices, this is the required format for the index buffer.
    /// This has no effect on non-indexed or non-strip draws.
    ///
    /// Specifying this value enables primitive restart, allowing individual strips to be separated
    /// with the index value `0xFFFF` when using `Uint16`, or `0xFFFFFFFF` when using `Uint32`.
    pub strip_index_format: Option<IndexFormat>,
    /// The face to consider the front for the purpose of culling and stencil operations.
    pub front_face: FrontFace,
    /// The face culling mode.
    pub cull_mode: Option<Face>,
    /// If set to true, the polygon depth is not clipped to 0-1 before rasterization.
    ///
    /// Enabling this requires `Features::DEPTH_CLIP_CONTROL` to be enabled.
    pub unclipped_depth: bool,
    /// Controls the way each polygon is rasterized. Can be either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to `Line` requires `Features::POLYGON_MODE_LINE` to be enabled.
    ///
    /// Setting this to `Point` requires `Features::POLYGON_MODE_POINT` to be enabled.
    pub polygon_mode: PolygonMode,
    /// If set to true, the primitives are rendered with conservative overestimation. I.e. any rastered pixel touched by it is filled.
    /// Only valid for PolygonMode::Fill!
    ///
    /// Enabling this requires `Features::CONSERVATIVE_RASTERIZATION` to be enabled.
    pub conservative: bool,
}

impl From<PrimitiveState> for RawPrimitiveState {
    fn from(value: PrimitiveState) -> Self {
        RawPrimitiveState {
            topology: value.topology.into(),
            strip_index_format: value.strip_index_format.map(Into::into),
            front_face: value.front_face.into(),
            cull_mode: value.cull_mode.map(Into::into),
            unclipped_depth: value.unclipped_depth,
            polygon_mode: value.polygon_mode.into(),
            conservative: value.conservative,
        }
    }
}
