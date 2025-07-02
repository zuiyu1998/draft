pub mod layout_cache;

pub use layout_cache::*;

use std::{borrow::Cow, sync::Arc};

use crate::{
    MaterialData,
    gfx_base::{
        CachedPipelineId, GetPipelineCache, Pipeline, PipelineCache, RenderDevice,
        VertexBufferLayout,
    },
};

use wgpu::{ShaderModuleDescriptor, ShaderSource};

use fyrox_core::log::Log;

use crate::{
    FrameworkError, MaterialResource, PipelineDescriptor, Shader, ShaderResource, TemporaryCache,
};

pub struct ShaderModuleData {
    pub module: Arc<wgpu::ShaderModule>,
}

impl ShaderModuleData {
    pub fn new(
        composer: &mut naga_oil::compose::Composer,
        device: &RenderDevice,
        shader: &Shader,
    ) -> Result<Self, FrameworkError> {
        let naga = composer.make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            ..(&shader.definition).into()
        })?;

        let shader_source = ShaderSource::Naga(Cow::Owned(naga));

        let module_descriptor = ShaderModuleDescriptor {
            label: None,
            source: shader_source,
        };

        let shader_module = device.wgpu_device().create_shader_module(module_descriptor);

        Ok(ShaderModuleData {
            module: Arc::new(shader_module),
        })
    }
}

#[derive(Default)]
pub struct ShaderCache {
    composer: naga_oil::compose::Composer,
    cache: TemporaryCache<ShaderModuleData>,
}

impl ShaderCache {
    pub fn remove(&mut self, shader: &ShaderResource) {
        let mut state = shader.state();
        if let Some(shader_state) = state.data() {
            self.cache.remove(&shader_state.cache_index);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.cache.update(dt)
    }

    pub fn get(
        &mut self,
        device: &RenderDevice,
        shader: &ShaderResource,
    ) -> Result<&wgpu::ShaderModule, FrameworkError> {
        let mut shader_state = shader.state();

        if let Some(shader_state) = shader_state.data() {
            match self.cache.get_or_insert_with(
                &shader_state.cache_index,
                Default::default(),
                || ShaderModuleData::new(&mut self.composer, device, shader_state),
            ) {
                Ok(data) => Ok(&data.module),
                Err(error) => Err(error),
            }
        } else {
            Err(FrameworkError::ShaderNotLoaded(shader.clone()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub pipeline: Pipeline,
}

#[derive(Default)]
pub struct MaterialStorage {
    pub shader_cache: ShaderCache,
    pub pipeline_layout_cache: PipelineLayoutCache,
    pub material_cache: TemporaryCache<MaterialData>,
}

impl MaterialStorage {
    pub fn get_or_insert(
        &mut self,
        device: &RenderDevice,
        material: &MaterialResource,
        layouts: &[VertexBufferLayout],
    ) -> Option<&MaterialData> {
        let mut material_state = material.state();

        if let Some(material_state) = material_state.data() {
            match self.material_cache.get_mut_or_insert_with(
                &material_state.cache_index,
                Default::default(),
                || {
                    MaterialData::prepare(
                        &material_state.definition,
                        device,
                        layouts,
                        &mut self.shader_cache,
                        &mut self.pipeline_layout_cache,
                    )
                },
            ) {
                Ok(data) => {
                    data.set_cached_pipeline_id(CachedPipelineId::new(
                        material_state.cache_index.get(),
                    ));
                    Some(data)
                }
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

impl GetPipelineCache for MaterialStorage {
    fn get_pipeline_cache(&self) -> PipelineCache {
        let mut target = vec![];
        for index in 0..self.material_cache.buffer.len() {
            let pipeline = self
                .material_cache
                .buffer
                .get_raw(index)
                .map(|entry| entry.value.get_pipeline());

            target.push(pipeline);
        }

        PipelineCache::new(target)
    }
}
