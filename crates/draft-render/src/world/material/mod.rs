use downcast_rs::{Downcast, impl_downcast};
use fyrox_core::{
    TypeUuidProvider, Uuid, log::Log, reflect::*, sparse::AtomicIndex, uuid, visitor::*,
};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path, sync::Arc};

use crate::{
    FrameworkError, PipelineCache, PipelineDescriptor, RenderPipelineDescriptor, TemporaryCache,
    gfx_base::CachedPipelineId,
};

pub type MaterialResource = Resource<Material>;

pub struct MaterialData(PipelineDataContainer);

impl MaterialData {
    pub fn new(value: PipelineDataContainer) -> Self {
        MaterialData(value)
    }

    pub fn pipeline_data_ref<T: PipelineData>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }

    pub fn from_material(
        material: &Material,
        pipeline_cache: &mut PipelineCache,
        desc: &PipelineDescriptor,
    ) -> Result<Self, FrameworkError> {
        let value = material.definition.0.prepare(pipeline_cache, desc)?;

        Ok(MaterialData::new(value))
    }
}

#[derive(Default)]
pub struct MaterialCache {
    cache: TemporaryCache<MaterialData>,
}

impl MaterialCache {
    pub fn get<T: PipelineData>(&self, material: &MaterialResource) -> Option<&T> {
        let material_state = material.state();
        if let Some(material_state) = material_state.data_ref() {
            self.cache
                .buffer
                .get(&material_state.cache_index)
                .and_then(|entry| entry.pipeline_data_ref())
        } else {
            None
        }
    }

    pub fn get_or_create(
        &mut self,
        material: &MaterialResource,
        desc: &PipelineDescriptor,
        pipeline_cache: &mut PipelineCache,
    ) -> Option<&MaterialData> {
        let material_state = material.state();
        if let Some(material_state) = material_state.data_ref() {
            match self.cache.get_or_insert_with(
                &material_state.cache_index,
                Default::default(),
                || MaterialData::from_material(material_state, pipeline_cache, desc),
            ) {
                Ok(data) => Some(data),
                Err(error) => {
                    Log::err(format!("{error}"));

                    None
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "3cee68e7-ef0a-463b-a2f5-68f90586b654")]
pub struct Material {
    pub definition: MaterialDefinition,
    #[reflect(hidden)]
    #[visit(skip)]
    pub cache_index: Arc<AtomicIndex>,
    #[visit(optional)]
    pub modifications_counter: u64,
}

impl Material {
    pub fn new(definition: MaterialDefinition) -> Self {
        Material {
            definition,
            cache_index: Default::default(),
            modifications_counter: 0,
        }
    }
}

#[derive(Debug)]
pub struct MaterialDefinition(Box<dyn ErasedMaterial>);

impl MaterialDefinition {
    pub fn specialize(&self, desc: &mut PipelineDescriptor) {
        self.0.specialize(desc);
    }
}

impl MaterialDefinition {
    pub fn new<T: RenderMaterial>(value: T) -> Self {
        Self(Box::new(value))
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
        MaterialDefinition::new(MaterialBase)
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

pub trait PipelineData: 'static + Downcast + Clone {}

impl PipelineData for CachedPipelineId {}

pub trait ErasedPipelineData: 'static + Downcast {
    fn clone_box(&self) -> Box<dyn ErasedPipelineData>;
}

impl<T: PipelineData> ErasedPipelineData for T {
    fn clone_box(&self) -> Box<dyn ErasedPipelineData> {
        Box::new(self.clone())
    }
}

impl_downcast!(ErasedPipelineData);

pub trait RenderMaterial:
    'static + Debug + Clone + Reflect + Visit + Default + Send + Sync
{
    type PipelineData: PipelineData;

    fn specialize(&self, desc: &mut RenderPipelineDescriptor);

    fn prepare(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &RenderPipelineDescriptor,
    ) -> Result<Self::PipelineData, FrameworkError>;
}

pub struct PipelineDataContainer(Box<dyn ErasedPipelineData>);

impl PipelineDataContainer {
    pub fn new<T: PipelineData>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub fn downcast_ref<T: PipelineData>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

pub trait ErasedMaterial: 'static + Debug + Reflect + Visit + Send + Sync {
    fn clone_box(&self) -> Box<dyn ErasedMaterial>;

    fn specialize(&self, desc: &mut PipelineDescriptor);

    fn prepare(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError>;
}

impl<T: RenderMaterial> ErasedMaterial for T {
    fn clone_box(&self) -> Box<dyn ErasedMaterial> {
        Box::new(self.clone())
    }

    fn prepare(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError> {
        match desc {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => {
                let pipeline_data = <T as RenderMaterial>::prepare(self, pipeline_cache, desc)?;

                Ok(PipelineDataContainer::new(pipeline_data))
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn specialize(&self, desc: &mut PipelineDescriptor) {
        match desc {
            PipelineDescriptor::RenderPipelineDescriptor(desc) => {
                <T as RenderMaterial>::specialize(self, desc);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Reflect, Visit, Clone, Default)]
pub struct MaterialBase;

impl RenderMaterial for MaterialBase {
    type PipelineData = CachedPipelineId;

    fn prepare(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &RenderPipelineDescriptor,
    ) -> Result<Self::PipelineData, FrameworkError> {
        let desc = PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc.clone()));
        Ok(pipeline_cache.get_or_create(&desc))
    }

    fn specialize(&self, _desc: &mut RenderPipelineDescriptor) {}
}
