use draft_shader::{Shader, ShaderResource};

use crate::{
    FrameworkError,
    render_world::{ResourceId, TemporaryCache},
};

pub struct ShaderRenderData {}

pub fn create_shader_render_data(_shader: &Shader) -> Result<ShaderRenderData, FrameworkError> {
    todo!()
}

pub struct ShaderCache {
    cache: TemporaryCache<ShaderRenderData>,
}

impl ShaderCache {
    pub fn get_create_mesh(
        &mut self,
        shader: &ShaderResource,
    ) -> Result<ResourceId<Shader>, FrameworkError> {
        if !shader.is_ok() {
            return Err(FrameworkError::ShaderNotLoaded);
        }

        let shader = shader.data_ref();

        match self
            .cache
            .get_mut_or_insert_with(&shader.cache_index, Default::default(), || {
                create_shader_render_data(&shader)
            }) {
            Ok(_) => Ok(ResourceId::new(shader.cache_index.get())),
            Err(e) => {
                return Err(e);
            }
        }
    }
}
