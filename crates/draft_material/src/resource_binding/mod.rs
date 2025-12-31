use fxhash::FxHashMap;
use fyrox_core::{
    ImmutableString,
    algebra::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4},
    color::Color,
    reflect::*,
    visitor::*,
};
use strum_macros::{AsRefStr, EnumString, VariantNames};

#[derive(Debug, Visit, Clone, Reflect, AsRefStr, EnumString, VariantNames)]
pub enum MaterialProperty {
    /// Real number.
    Float(f32),

    /// Real number array.
    FloatArray(Vec<f32>),

    /// Integer number.
    Int(i32),

    /// Integer number array.
    IntArray(Vec<i32>),

    /// Natural number.
    UInt(u32),

    /// Natural number array.
    UIntArray(Vec<u32>),

    /// Two-dimensional vector.
    Vector2(Vector2<f32>),

    /// Two-dimensional vector array.
    Vector2Array(Vec<Vector2<f32>>),

    /// Three-dimensional vector.
    Vector3(Vector3<f32>),

    /// Three-dimensional vector array.
    Vector3Array(Vec<Vector3<f32>>),

    /// Four-dimensional vector.
    Vector4(Vector4<f32>),

    /// Four-dimensional vector array.
    Vector4Array(Vec<Vector4<f32>>),

    /// 2x2 Matrix.
    Matrix2(Matrix2<f32>),

    /// 2x2 Matrix array.
    Matrix2Array(Vec<Matrix2<f32>>),

    /// 3x3 Matrix.
    Matrix3(Matrix3<f32>),

    /// 3x3 Matrix array.
    Matrix3Array(Vec<Matrix3<f32>>),

    /// 4x4 Matrix.
    Matrix4(Matrix4<f32>),

    /// 4x4 Matrix array.
    Matrix4Array(Vec<Matrix4<f32>>),

    /// Boolean value.
    Bool(bool),

    /// An sRGB color.
    Color(Color),
}

impl Default for MaterialProperty {
    fn default() -> Self {
        Self::Float(0.0)
    }
}

#[derive(Default, Debug, Visit, Clone, Reflect)]
pub struct MaterialPropertyGroup {
    properties: FxHashMap<ImmutableString, MaterialProperty>,
}

#[derive(Debug, Clone, Reflect, Visit, AsRefStr, EnumString, VariantNames)]
pub enum MaterialResourceBinding {
    /// A texture.
    // Texture(MaterialTextureBinding),
    /// A group of properties.
    PropertyGroup(MaterialPropertyGroup),
}

impl Default for MaterialResourceBinding {
    fn default() -> Self {
        Self::PropertyGroup(Default::default())
    }
}
