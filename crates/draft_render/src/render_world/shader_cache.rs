use draft_graphics::{RenderDevice, ShaderModule, ShaderModuleDescriptor};
use draft_shader::{Shader, ShaderResource};

use crate::{
    FrameworkError,
    render_world::{ResourceId, TemporaryCache},
};

pub struct ShaderRenderData {
    pub shader_module: ShaderModule,
    pub modifications_counter: u64,
}

pub fn create_shader_render_data(
    device: &RenderDevice,
    shader: &Shader,
) -> Result<ShaderRenderData, FrameworkError> {
    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: shader.source.get_shader_soource(),
    });

    Ok(ShaderRenderData {
        shader_module,
        modifications_counter: shader.modifications_counter,
    })
}

pub struct ShaderCache {
    cache: TemporaryCache<ShaderRenderData>,
}

impl ShaderCache {
    pub fn get_create_shader(
        &mut self,
        device: &RenderDevice,
        shader: &ShaderResource,
    ) -> Result<ResourceId<Shader>, FrameworkError> {
        if !shader.is_ok() {
            return Err(FrameworkError::ShaderNotLoaded);
        }

        let shader = shader.data_ref();

        match self
            .cache
            .get_mut_or_insert_with(&shader.cache_index, Default::default(), || {
                create_shader_render_data(device, &shader)
            }) {
            Ok(shader_render_data) => {
                if shader_render_data.modifications_counter != shader.modifications_counter {
                    *shader_render_data = create_shader_render_data(device, &shader)?;
                }

                Ok(ResourceId::new(shader.cache_index.get()))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
