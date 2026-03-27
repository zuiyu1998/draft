use std::sync::Arc;

use draft_core::{RenderResource, collections::FxHashMap};
use draft_image::ImageResource;
use fyrox_core::{
    ImmutableString, TypeUuidProvider, Uuid, algebra::*, color::Color, reflect::*,
    sparse::AtomicIndex, uuid, visitor::*,
};
use fyrox_resource::{Resource, ResourceData};

use crate::PipelineResource;

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Visit, Clone, Reflect)]
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

#[derive(Debug, Reflect, Clone)]
pub struct MaterialTextureBinding {
    pub value: Option<ImageResource>,
}

#[derive(Debug, Reflect, Clone)]
pub enum MaterialResourceBinding {
    /// A texture.
    Texture(MaterialTextureBinding),
    /// A group of properties.
    PropertyGroup(MaterialPropertyGroup),
}

#[derive(Debug, Reflect, Clone)]
pub struct Material {
    pipeline: PipelineResource,
    resource_bindings: FxHashMap<ImmutableString, MaterialResourceBinding>,

    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pipeline: PipelineResource::default(),
            resource_bindings: Default::default(),
            cache_index: Default::default(),
        }
    }
}

impl RenderResource for Material {
    fn get_cache_index(&self) -> &Arc<AtomicIndex> {
        &self.cache_index
    }
}

impl TypeUuidProvider for Material {
    fn type_uuid() -> Uuid {
        uuid!("e1ce1983-4e80-4d8b-a4e5-9b05112e3b5c")
    }
}

impl Visit for Material {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        Ok(())
    }
}

impl ResourceData for Material {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
