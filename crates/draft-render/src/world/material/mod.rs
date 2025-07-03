pub mod cache;

pub use cache::*;
use downcast_rs::{Downcast, impl_downcast};

use std::{error::Error, fmt::Debug, path::Path, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

use crate::{
    FrameworkError, ShaderResource,
    gfx_base::{
        BindGroupLayoutEntry, CachedPipelineId, ColorTargetState, DepthStencilState,
        MultisampleState, Pipeline, PipelineCompilationOptions, PrimitiveState, RawFragmentState,
        RawRenderPipelineDescriptor, RawVertexAttribute, RawVertexBufferLayout, RawVertexState,
        RenderDevice, RenderPipeline, VertexBufferLayout,
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

    pub fn update_vertex_buffer_layouts(&mut self, layouts: &[VertexBufferLayout]) {
        self.0.update_vertex_buffer_layouts(layouts);
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

pub struct MaterialData {
    id: CachedPipelineId,
    cached_pipeline: CachedPipeline,
    data: Box<dyn PipelineData>,
}

impl MaterialData {
    pub fn new<T: PipelineData>(
        id: CachedPipelineId,
        cached_pipeline: CachedPipeline,
        value: T,
    ) -> Self {
        MaterialData {
            cached_pipeline,
            id,
            data: Box::new(value),
        }
    }

    pub fn downcast<T: PipelineData>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }

    fn get_pipeline(&self) -> Pipeline {
        self.cached_pipeline.pipeline.clone()
    }

    pub fn get_cached_pipeline_id(&self) -> CachedPipelineId {
        self.id
    }

    pub fn set_cached_pipeline_id(&mut self, id: CachedPipelineId) {
        self.id = id;
    }

    pub fn prepare(
        material: &MaterialDefinition,
        device: &RenderDevice,
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Result<Self, FrameworkError> {
        material
            .0
            .prepare(device, shader_cache, pipeline_layout_cache)
    }
}

pub fn crate_pipeline_with_render_pipeline_descriptor(
    device: &RenderDevice,
    desc: &RenderPipelineDescriptor,
    shader_cache: &mut ShaderCache,
    pipeline_layout_cache: &mut PipelineLayoutCache,
) -> Result<CachedPipeline, FrameworkError> {
    let vertex_module = shader_cache.get(device, &desc.vertex.shader)?.clone();
    let fragment_module = match &desc.fragment {
        Some(fragment) => match shader_cache.get(device, &fragment.shader) {
            Ok(module) => Some(module.clone()),
            Err(err) => return Err(err),
        },
        None => None,
    };

    let layout = pipeline_layout_cache.get(device, &desc.layout)?.clone();

    let vertex_buffer_layouts = desc
        .vertex
        .buffers
        .iter()
        .map(|layout| {
            (
                layout.array_stride,
                layout
                    .attributes
                    .iter()
                    .map(|attribute| attribute.into())
                    .collect::<Vec<RawVertexAttribute>>(),
                layout.step_mode,
            )
        })
        .collect::<Vec<_>>();
    let vertex_buffer_layouts = vertex_buffer_layouts
        .iter()
        .map(
            |(array_stride, attributes, step_mode)| RawVertexBufferLayout {
                array_stride: *array_stride,
                attributes,
                step_mode: (*step_mode).into(),
            },
        )
        .collect::<Vec<_>>();

    let fragment_data = desc.fragment.clone().map(|fragment| {
        (
            fragment_module.unwrap(),
            fragment.entry_point,
            fragment
                .targets
                .iter()
                .map(|target| target.as_ref().map(|target| target.into()))
                .collect::<Vec<_>>(),
            fragment.compilation_options,
        )
    });

    let descriptor = RawRenderPipelineDescriptor {
        multiview: None,
        depth_stencil: desc
            .depth_stencil
            .as_ref()
            .map(|depth_stencil| depth_stencil.into()),
        label: Some(&desc.label),
        layout: Some(&layout),
        multisample: desc.multisample.into(),
        primitive: desc.primitive.into(),
        vertex: RawVertexState {
            buffers: &vertex_buffer_layouts,
            entry_point: desc.vertex.entry_point.as_deref(),
            module: &vertex_module,
            compilation_options: desc.vertex.compilation_options.get_raw(),
        },
        fragment: fragment_data.as_ref().map(
            |(module, entry_point, targets, compilation_options)| RawFragmentState {
                entry_point: entry_point.as_deref(),
                module,
                targets,
                compilation_options: compilation_options.get_raw(),
            },
        ),
        cache: None,
    };

    let pipeline = device.wgpu_device().create_render_pipeline(&descriptor);

    Ok(CachedPipeline {
        descriptor: PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc.clone())),
        pipeline: Pipeline::RenderPipeline(RenderPipeline::new(pipeline)),
    })
}

pub trait PipelineData: 'static + Downcast {}

impl_downcast!(PipelineData);

impl PipelineData for () {}

impl RenderMaterial for RenderPipelineDescriptor {
    type PipelineData = ();

    fn specialize(&self) -> RenderPipelineDescriptor {
        self.clone()
    }

    fn prepare(
        &self,
        _device: &RenderDevice,
        _shader_cache: &mut ShaderCache,
        _pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Result<Self::PipelineData, FrameworkError> {
        Ok(())
    }
}

pub trait RenderMaterial:
    'static + Debug + Clone + Reflect + Visit + Default + Send + Sync
{
    type PipelineData: PipelineData;

    fn specialize(&self) -> RenderPipelineDescriptor;

    fn prepare(
        &self,
        device: &RenderDevice,
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Result<Self::PipelineData, FrameworkError>;

    fn update_vertex_buffer_layouts(&mut self, _layouts: &[VertexBufferLayout]) {}
}

pub trait ErasedRenderMaterial: 'static + Debug + Reflect + Visit + Send + Sync {
    fn prepare(
        &self,
        device: &RenderDevice,
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Result<MaterialData, FrameworkError>;

    fn clone_box(&self) -> Box<dyn ErasedRenderMaterial>;

    fn update_vertex_buffer_layouts(&mut self, layouts: &[VertexBufferLayout]);
}

impl<T: RenderMaterial> ErasedRenderMaterial for T {
    fn prepare(
        &self,
        device: &RenderDevice,
        shader_cache: &mut ShaderCache,
        pipeline_layout_cache: &mut PipelineLayoutCache,
    ) -> Result<MaterialData, FrameworkError> {
        let desc = self.specialize();

        let cached_pipeline = crate_pipeline_with_render_pipeline_descriptor(
            device,
            &desc,
            shader_cache,
            pipeline_layout_cache,
        )?;

        let data = self.prepare(device, shader_cache, pipeline_layout_cache)?;
        Ok(MaterialData::new(
            CachedPipelineId::default(),
            cached_pipeline,
            data,
        ))
    }

    fn clone_box(&self) -> Box<dyn ErasedRenderMaterial> {
        Box::new(self.clone())
    }

    fn update_vertex_buffer_layouts(&mut self, layouts: &[VertexBufferLayout]) {
        <T as RenderMaterial>::update_vertex_buffer_layouts(self, layouts);
    }
}
