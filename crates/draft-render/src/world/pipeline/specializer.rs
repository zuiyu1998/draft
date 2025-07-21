use downcast_rs::{Downcast, impl_downcast};
use fyrox_resource::{Resource, ResourceData};
use std::{error::Error, fmt::Debug, path::Path, sync::Arc};

use super::{PipelineCache, PipelineDescriptor, RenderPipelineDescriptor};
use crate::{FrameworkError, TemporaryCache, gfx_base::CachedPipelineId};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};

pub type PipelineSpecializerResource = Resource<PipelineSpecializer>;

#[derive(Default)]
pub struct PipelineSpecializerCache {
    cache: TemporaryCache<PipelineSpecializerData>,
}

pub struct PipelineSpecializerData {
    data: PipelineDataContainer,
    pub modifications_counter: u64,
}

impl PipelineSpecializerData {
    pub fn new(
        desc: &mut PipelineDescriptor,
        pipeline_cache: &mut PipelineCache,
        definition: &PipelineSpecializerDefinition,
        modifications_counter: u64,
    ) -> Result<Self, FrameworkError> {
        let data = definition.create_pipeline_data(pipeline_cache, desc)?;

        Ok(PipelineSpecializerData {
            data,
            modifications_counter,
        })
    }

    pub fn data_ref<T: PipelineData>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
}

impl PipelineSpecializerCache {
    pub fn get_or_create<T: PipelineData>(
        &mut self,
        desc: &mut PipelineDescriptor,
        pipeline_cache: &mut PipelineCache,
        specializer: &PipelineSpecializerResource,
    ) -> Result<(), FrameworkError> {
        let specializer_state = specializer.state();
        if let Some(specializer_state) = specializer_state.data_ref() {
            let data = self.cache.get_mut_or_insert_with(
                &specializer_state.cache_index,
                Default::default(),
                || {
                    PipelineSpecializerData::new(
                        desc,
                        pipeline_cache,
                        &specializer_state.definition,
                        specializer_state.modifications_counter,
                    )
                },
            )?;

            if data.modifications_counter != specializer_state.modifications_counter {
                *data = PipelineSpecializerData::new(
                    desc,
                    pipeline_cache,
                    &specializer_state.definition,
                    specializer_state.modifications_counter,
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Reflect, Visit, Default, TypeUuidProvider)]
#[type_uuid(id = "b4c3e37b-5150-4228-a7fb-c29b07a03e2f")]
pub struct PipelineSpecializer {
    pub definition: PipelineSpecializerDefinition,
    #[reflect(hidden)]
    #[visit(skip)]
    cache_index: Arc<AtomicIndex>,
    #[visit(optional)]
    modifications_counter: u64,
}

impl PipelineSpecializer {
    pub fn new_render_specializer<T: RenderPipelineSpecializer>(render_specializer: T) -> Self {
        PipelineSpecializer {
            definition: PipelineSpecializerDefinition::new(render_specializer),
            cache_index: Default::default(),
            modifications_counter: 0,
        }
    }
}

impl ResourceData for PipelineSpecializer {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut visitor = Visitor::new();
        self.visit("PipelineSpecializer", &mut visitor)?;
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
pub struct PipelineSpecializerDefinition(Box<dyn ErasedPipelineSpecializer>);

impl PipelineSpecializerDefinition {
    pub fn new<T: ErasedPipelineSpecializer>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub fn create_pipeline_data(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &mut PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError> {
        self.0.create_pipeline_data(pipeline_cache, desc)
    }
}

impl Clone for PipelineSpecializerDefinition {
    fn clone(&self) -> Self {
        PipelineSpecializerDefinition(self.0.clone_box())
    }
}

impl Visit for PipelineSpecializerDefinition {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.0.visit(name, visitor)
    }
}

impl Reflect for PipelineSpecializerDefinition {
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

impl Default for PipelineSpecializerDefinition {
    fn default() -> Self {
        PipelineSpecializerDefinition::new(RenderPipelineDescriptor::default())
    }
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

impl Clone for PipelineDataContainer {
    fn clone(&self) -> Self {
        PipelineDataContainer(self.0.clone_box())
    }
}

pub trait PipelineData: 'static + Clone {}

impl PipelineData for CachedPipelineId {}

pub trait ErasedPipelineData: 'static + Downcast {
    fn clone_box(&self) -> Box<dyn ErasedPipelineData>;
}

impl_downcast!(ErasedPipelineData);

impl<T: PipelineData> ErasedPipelineData for T {
    fn clone_box(&self) -> Box<dyn ErasedPipelineData> {
        Box::new(self.clone())
    }
}

pub trait RenderPipelineSpecializer:
    'static + Debug + Reflect + Visit + Send + Sync + Clone
{
    type Data: PipelineData;

    fn specialize(&self, desc: &mut RenderPipelineDescriptor);

    fn create_pipeline_data(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &mut PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError>;
}

impl RenderPipelineSpecializer for RenderPipelineDescriptor {
    type Data = CachedPipelineId;

    fn specialize(&self, desc: &mut RenderPipelineDescriptor) {
        *desc = self.clone();
    }

    fn create_pipeline_data(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &mut PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError> {
        if let Some(desc) = desc.render_pipeline_descriptor() {
            self.specialize(desc);
        }

        let id = pipeline_cache.get_or_create(desc);

        Ok(PipelineDataContainer::new(id))
    }
}

pub trait ErasedPipelineSpecializer: 'static + Debug + Reflect + Visit + Send + Sync {
    fn clone_box(&self) -> Box<dyn ErasedPipelineSpecializer>;

    fn create_pipeline_data(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &mut PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError>;
}

impl<T: RenderPipelineSpecializer> ErasedPipelineSpecializer for T {
    fn clone_box(&self) -> Box<dyn ErasedPipelineSpecializer> {
        Box::new(self.clone())
    }

    fn create_pipeline_data(
        &self,
        pipeline_cache: &mut PipelineCache,
        desc: &mut PipelineDescriptor,
    ) -> Result<PipelineDataContainer, FrameworkError> {
        <T as RenderPipelineSpecializer>::create_pipeline_data(self, pipeline_cache, desc)
    }
}
