mod named_value;

use crate::{PipelineSpecializerResource, Std140, TextureResource};
use bytes::BufMut;
use fxhash::FxHashMap;
use fyrox_core::{
    ImmutableString, TypeUuidProvider, Uuid,
    algebra::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4},
    color::Color,
    reflect::*,
    uuid,
    visitor::*,
};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path};
use strum_macros::{AsRefStr, EnumString, VariantNames};

pub use named_value::*;
pub type MaterialResource = Resource<Material>;

pub struct PropertyGroup<'a, const N: usize> {
    pub properties: [NamedValue<MaterialPropertyRef<'a>>; N],
}

impl<'a, const N: usize> Std140 for PropertyGroup<'a, N> {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        for property in self.properties.iter() {
            property.value.write(dest, size);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialPropertyRef<'a> {
    /// Real number.
    Float(&'a f32),

    /// Real number array.
    FloatArray(&'a [f32]),

    /// Integer number.
    Int(&'a i32),

    /// Integer number array.
    IntArray(&'a [i32]),

    /// Natural number.
    UInt(&'a u32),

    /// Natural number array.
    UIntArray(&'a [u32]),

    /// Two-dimensional vector.
    Vector2(&'a Vector2<f32>),

    /// Two-dimensional vector array.
    Vector2Array(&'a [Vector2<f32>]),

    /// Three-dimensional vector.
    Vector3(&'a Vector3<f32>),

    /// Three-dimensional vector array.
    Vector3Array(&'a [Vector3<f32>]),

    /// Four-dimensional vector.
    Vector4(&'a Vector4<f32>),

    /// Four-dimensional vector array.
    Vector4Array(&'a [Vector4<f32>]),

    /// 2x2 Matrix.
    Matrix2(&'a Matrix2<f32>),

    /// 2x2 Matrix array.
    Matrix2Array(&'a [Matrix2<f32>]),

    /// 3x3 Matrix.
    Matrix3(&'a Matrix3<f32>),

    /// 3x3 Matrix array.
    Matrix3Array(&'a [Matrix3<f32>]),

    /// 4x4 Matrix.
    Matrix4(&'a Matrix4<f32>),

    /// 4x4 Matrix array.
    Matrix4Array(&'a [Matrix4<f32>]),

    /// Boolean value.
    Bool(&'a bool),

    /// An sRGB color.
    Color(&'a Color),
}

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

impl<'a> Std140 for MaterialPropertyRef<'a> {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        match self {
            MaterialPropertyRef::Bool(v) => v.write(dest, size),
            MaterialPropertyRef::Float(v) => v.write(dest, size),
            MaterialPropertyRef::FloatArray(v) => v.write(dest, size),
            MaterialPropertyRef::Int(v) => v.write(dest, size),
            MaterialPropertyRef::IntArray(v) => v.write(dest, size),
            MaterialPropertyRef::UInt(v) => v.write(dest, size),
            MaterialPropertyRef::UIntArray(v) => v.write(dest, size),
            MaterialPropertyRef::Vector2(v) => v.write(dest, size),
            MaterialPropertyRef::Vector2Array(v) => v.write(dest, size),
            MaterialPropertyRef::Vector3(v) => v.write(dest, size),
            MaterialPropertyRef::Vector3Array(v) => v.write(dest, size),
            MaterialPropertyRef::Vector4(v) => v.write(dest, size),
            MaterialPropertyRef::Vector4Array(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix2(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix2Array(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix3(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix3Array(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix4(v) => v.write(dest, size),
            MaterialPropertyRef::Matrix4Array(v) => v.write(dest, size),
            MaterialPropertyRef::Color(v) => v.write(dest, size),
        }
    }
}

impl Default for MaterialProperty {
    fn default() -> Self {
        Self::Float(0.0)
    }
}

impl MaterialProperty {
    pub fn as_ref(&self) -> MaterialPropertyRef<'_> {
        match self {
            MaterialProperty::Float(v) => MaterialPropertyRef::Float(v),
            MaterialProperty::FloatArray(v) => MaterialPropertyRef::FloatArray(v),
            MaterialProperty::Int(v) => MaterialPropertyRef::Int(v),
            MaterialProperty::IntArray(v) => MaterialPropertyRef::IntArray(v),
            MaterialProperty::UInt(v) => MaterialPropertyRef::UInt(v),
            MaterialProperty::UIntArray(v) => MaterialPropertyRef::UIntArray(v),
            MaterialProperty::Vector2(v) => MaterialPropertyRef::Vector2(v),
            MaterialProperty::Vector2Array(v) => MaterialPropertyRef::Vector2Array(v),
            MaterialProperty::Vector3(v) => MaterialPropertyRef::Vector3(v),
            MaterialProperty::Vector3Array(v) => MaterialPropertyRef::Vector3Array(v),
            MaterialProperty::Vector4(v) => MaterialPropertyRef::Vector4(v),
            MaterialProperty::Vector4Array(v) => MaterialPropertyRef::Vector4Array(v),
            MaterialProperty::Matrix2(v) => MaterialPropertyRef::Matrix2(v),
            MaterialProperty::Matrix2Array(v) => MaterialPropertyRef::Matrix2Array(v),
            MaterialProperty::Matrix3(v) => MaterialPropertyRef::Matrix3(v),
            MaterialProperty::Matrix3Array(v) => MaterialPropertyRef::Matrix3Array(v),
            MaterialProperty::Matrix4(v) => MaterialPropertyRef::Matrix4(v),
            MaterialProperty::Matrix4Array(v) => MaterialPropertyRef::Matrix4Array(v),
            MaterialProperty::Bool(v) => MaterialPropertyRef::Bool(v),
            MaterialProperty::Color(v) => MaterialPropertyRef::Color(v),
        }
    }
}

#[derive(Debug, Clone, Visit, Reflect, Default)]
pub struct MaterialTextureBinding {
    value: Option<TextureResource>,
}

#[derive(Default, Debug, Visit, Clone, Reflect)]
pub struct MaterialPropertyGroup {
    properties: FxHashMap<ImmutableString, MaterialProperty>,
}

#[derive(Debug, Clone, AsRefStr, EnumString, VariantNames, Visit, Reflect)]
pub enum MaterialResourceBinding {
    /// A texture.
    Texture(MaterialTextureBinding),
    /// A group of properties.
    PropertyGroup(MaterialPropertyGroup),
}

impl Default for MaterialResourceBinding {
    fn default() -> Self {
        MaterialResourceBinding::PropertyGroup(Default::default())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct Material {
    specializer: PipelineSpecializerResource,
    resource_bindings: FxHashMap<ImmutableString, Vec<MaterialResourceBinding>>,
}

impl Material {
    pub fn from_specializer(specializer: PipelineSpecializerResource) -> Self {
        Material::new(specializer, Default::default())
    }

    pub fn push_binding(
        &mut self,
        name: impl Into<ImmutableString>,
        binding: MaterialResourceBinding,
    ) {
        self.resource_bindings
            .entry(name.into())
            .or_default()
            .push(binding);
    }

    pub fn new(
        specializer: PipelineSpecializerResource,
        resource_bindings: FxHashMap<ImmutableString, Vec<MaterialResourceBinding>>,
    ) -> Self {
        Self {
            specializer,
            resource_bindings,
        }
    }
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("Material", &mut visitor)?;
        visitor.save_binary_to_file(path)?;
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
