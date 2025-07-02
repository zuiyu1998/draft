pub mod storage;

pub use storage::*;

use std::{error::Error, fmt::Debug, path::Path, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

use crate::{
    ShaderResource,
    gfx_base::{
        BindGroupLayoutEntry, ColorTargetState, DepthStencilState, MultisampleState,
        PipelineCompilationOptions, PrimitiveState, RenderDevice, VertexBufferLayout,
    },
};

use fyrox_resource::{Resource, ResourceData};

pub type MaterialResource = Resource<Material>;

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct VertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub compilation_options: PipelineCompilationOptions,
    pub buffers: Vec<VertexBufferLayout>,
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct FragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub compilation_options: PipelineCompilationOptions,
    pub targets: Vec<Option<ColorTargetState>>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Debug, Clone, Reflect, Visit, Default, PartialEq, Eq, Hash)]
pub struct PipelineLayoutDescriptor {
    pub bind_group_layouts: Vec<BindGroupLayoutDescriptor>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub layout: PipelineLayoutDescriptor,
    pub vertex: VertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState>,
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ComputePipelineDescriptor {}

#[derive(Debug, Clone, Reflect, Visit)]
pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
    ComputePipelineDescriptor(Box<ComputePipelineDescriptor>),
}

impl PipelineDescriptor {
    pub fn render_pipeline_descriptor(&mut self) -> Option<&mut RenderPipelineDescriptor> {
        match self {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => Some(desc),
            _ => None,
        }
    }
}

impl Default for PipelineDescriptor {
    fn default() -> Self {
        PipelineDescriptor::RenderPipelineDescriptor(Box::default())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3485bce7-7b74-4970-9bf0-2b4a897b06dd")]
pub struct Material {
    pub definition: MaterialDefinition,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
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

#[derive(Debug)]
pub struct MaterialDefinition(Box<dyn ErasedRenderMaterial>);

impl MaterialDefinition {
    pub fn new<T: RenderMaterial>(value: T) -> Self {
        MaterialDefinition(Box::new(value))
    }
}

impl Clone for MaterialDefinition {
    fn clone(&self) -> Self {
        MaterialDefinition(self.0.clone_box())
    }
}

impl Visit for MaterialDefinition {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.0.visit(name, visitor)
    }
}

impl Reflect for MaterialDefinition {
    fn source_path() -> &'static str
    where
        Self: Sized,
    {
        file!()
    }

    fn derived_types() -> &'static [std::any::TypeId]
    where
        Self: Sized,
    {
        &[]
    }

    fn try_clone_box(&self) -> Option<Box<dyn Reflect>> {
        Some(Box::new(self.clone()))
    }

    fn query_derived_types(&self) -> &'static [std::any::TypeId] {
        Self::derived_types()
    }

    fn type_name(&self) -> &'static str {
        self.0.type_name()
    }

    fn doc(&self) -> &'static str {
        self.0.doc()
    }

    fn fields_ref(&self, func: &mut dyn FnMut(&[FieldRef])) {
        self.0.fields_ref(func)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [FieldMut])) {
        self.0.fields_mut(func)
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        Reflect::into_any(self.0)
    }

    fn as_any(&self, func: &mut dyn FnMut(&dyn std::any::Any)) {
        Reflect::as_any(&(*self.0), func)
    }

    fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn std::any::Any)) {
        Reflect::as_any_mut(&mut (*self.0), func)
    }

    fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
        self.0.as_reflect(func)
    }

    fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
        self.0.as_reflect_mut(func)
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
        self.0.set(value)
    }

    fn assembly_name(&self) -> &'static str {
        self.0.assembly_name()
    }

    fn type_assembly_name() -> &'static str
    where
        Self: Sized,
    {
        env!("CARGO_PKG_NAME")
    }
}

impl Default for MaterialDefinition {
    fn default() -> Self {
        MaterialDefinition(Box::new(RenderPipelineDescriptor::default()))
    }
}

pub trait RenderMaterialData: 'static {}

impl RenderMaterialData for () {}

impl RenderMaterial for RenderPipelineDescriptor {
    type Data = ();

    fn specialize(&self, layouts: &[VertexBufferLayout]) -> RenderPipelineDescriptor {
        let mut desc = self.clone();

        desc.vertex.buffers = layouts.to_vec();

        desc
    }

    fn prepare(
        &self,
        _device: &RenderDevice,
        _pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Self::Data {
    }
}

pub type BoxedRenderMaterialData = Box<dyn RenderMaterialData>;

pub trait RenderMaterial:
    'static + Debug + Clone + Reflect + Visit + Default + Send + Sync
{
    type Data: RenderMaterialData;

    fn specialize(&self, layouts: &[VertexBufferLayout]) -> RenderPipelineDescriptor;

    fn prepare(
        &self,
        device: &RenderDevice,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Self::Data;
}

pub trait ErasedRenderMaterial: 'static + Debug + Reflect + Visit + Send + Sync {
    fn specialize(&self, layouts: &[VertexBufferLayout]) -> RenderPipelineDescriptor;

    fn prepare(
        &self,
        device: &RenderDevice,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> BoxedRenderMaterialData;

    fn clone_box(&self) -> Box<dyn ErasedRenderMaterial>;
}

impl<T: RenderMaterial> ErasedRenderMaterial for T {
    fn specialize(&self, layouts: &[VertexBufferLayout]) -> RenderPipelineDescriptor {
        <T as RenderMaterial>::specialize(self, layouts)
    }

    fn prepare(
        &self,
        device: &RenderDevice,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> BoxedRenderMaterialData {
        Box::new(<T as RenderMaterial>::prepare(
            self,
            device,
            pipeline_layout_cache,
        ))
    }

    fn clone_box(&self) -> Box<dyn ErasedRenderMaterial> {
        Box::new(self.clone())
    }
}
