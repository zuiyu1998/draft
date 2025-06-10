pub mod cache;
pub mod layout_cache;

pub use cache::*;
pub use layout_cache::*;

use std::{borrow::Cow, sync::Arc};

use frame_graph::{
    GetPipelineCache, PipelineCache, RenderDevice,
    wgpu::{self, ShaderModuleDescriptor, ShaderSource},
};
use fyrox_core::log::Log;
use thiserror::Error;

use crate::{Shader, ShaderResource};

pub struct ShaderModuleData {
    pub module: Arc<wgpu::ShaderModule>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
}

impl ShaderModuleData {
    pub fn new(
        composer: &mut naga_oil::compose::Composer,
        device: &RenderDevice,
        shader: &Shader,
    ) -> Result<Self, Error> {
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

pub struct ShaderCache {
    composer: naga_oil::compose::Composer,
    cache: TemporaryCache<ShaderModuleData>,
}

impl ShaderCache {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        shader: &ShaderResource,
    ) -> Option<Arc<wgpu::ShaderModule>> {
        let mut shader_state = shader.state();

        if let Some(shader_state) = shader_state.data() {
            match self.cache.get_or_insert_with(
                &shader_state.cache_index,
                Default::default(),
                || ShaderModuleData::new(&mut self.composer, device, shader_state),
            ) {
                Ok(data) => Some(data.module.clone()),
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

pub struct PipelineStorage {}

impl GetPipelineCache for PipelineStorage {
    fn get_pipeline_cache(&self) -> PipelineCache {
        todo!()
    }
}
