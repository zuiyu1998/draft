use std::{borrow::Cow, sync::Arc};

use crate::{FrameworkError, ShaderResource, TemporaryCache};
use draft_gfx_base::{RawShaderModuleDescriptor, RawShaderSource, RenderDevice};

use super::Shader;

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

        let shader_source = RawShaderSource::Naga(Cow::Owned(naga));

        let module_descriptor = RawShaderModuleDescriptor {
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
            Err(shader.clone().into())
        }
    }
}
