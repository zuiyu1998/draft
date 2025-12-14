use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    sync::Arc,
};

use draft_graphics::frame_graph::gfx_base::{CachedPipelineId, GpuShaderModule, RenderDevice};
use thiserror::Error;
use tracing::debug;

use crate::{Shader, ShaderDefVal, ShaderImport, ShaderResource};

#[derive(Debug, Error)]
pub enum ShaderCacheError {
    #[error(
        "Pipeline could not be compiled because the following shader could not be loaded: {0:?}"
    )]
    ShaderNotLoaded(String),
    #[error("Shader import not yet available.")]
    ShaderImportNotYetAvailable,
    #[error(transparent)]
    ProcessShaderError(#[from] naga_oil::compose::ComposerError),
}

type ShaderId = u64;

struct ShaderData {
    pipelines: HashSet<CachedPipelineId>,
    processed_shaders: HashMap<Box<[ShaderDefVal]>, Arc<GpuShaderModule>>,
    resolved_imports: HashMap<ShaderImport, ShaderId>,
    dependents: HashSet<ShaderId>,
}

impl Default for ShaderData {
    fn default() -> Self {
        Self {
            pipelines: Default::default(),
            processed_shaders: Default::default(),
            resolved_imports: Default::default(),
            dependents: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShaderCacheSource<'a> {
    SpirV(&'a [u8]),
    Wgsl(String),
    Naga(naga::Module),
}

pub struct ShaderCache {
    data: HashMap<ShaderId, ShaderData>,
    shaders: HashMap<ShaderId, Shader>,
    pub composer: naga_oil::compose::Composer,
    import_path_shaders: HashMap<ShaderImport, ShaderId>,
}

impl ShaderCache {
    pub fn new() -> Self {
        let composer = naga_oil::compose::Composer::default();

        Self {
            data: Default::default(),
            shaders: Default::default(),
            composer,
            import_path_shaders: Default::default(),
        }
    }

    fn add_import_to_composer(
        composer: &mut naga_oil::compose::Composer,
        import_path_shaders: &HashMap<ShaderImport, ShaderId>,
        shaders: &HashMap<ShaderId, Shader>,
        import: &ShaderImport,
    ) -> Result<(), ShaderCacheError> {
        // Early out if we've already imported this module
        if composer.contains_module(&import.module_name()) {
            return Ok(());
        }

        // Check if the import is available (this handles the recursive import case)
        let shader = import_path_shaders
            .get(import)
            .and_then(|handle| shaders.get(handle))
            .ok_or(ShaderCacheError::ShaderImportNotYetAvailable)?;

        // Recurse down to ensure all import dependencies are met
        for import in &shader.imports {
            Self::add_import_to_composer(composer, import_path_shaders, shaders, import)?;
        }

        let shader_defs = shader
            .shader_defs
            .iter()
            .map(|def| match def {
                ShaderDefVal::Bool(name, b) => {
                    (name.clone(), naga_oil::compose::ShaderDefValue::Bool(*b))
                }
                ShaderDefVal::Int(name, i) => {
                    (name.clone(), naga_oil::compose::ShaderDefValue::Int(*i))
                }
                ShaderDefVal::UInt(name, i) => {
                    (name.clone(), naga_oil::compose::ShaderDefValue::UInt(*i))
                }
            })
            .collect();

        let as_name = match &shader.import_path {
            ShaderImport::AssetPath(asset_path) => Some(format!("\"{asset_path}\"")),
            ShaderImport::Custom(_) => None,
        };

        let additional_imports = shader
            .additional_imports
            .iter()
            .map(|import| import.get_naga_import_definition())
            .collect::<Vec<_>>();

        let desc = naga_oil::compose::ComposableModuleDescriptor {
            source: shader.source.as_str(),
            file_path: &shader.path,
            language: (&shader.source).into(),
            additional_imports: &additional_imports,
            shader_defs,
            as_name,
        };

        composer.add_composable_module(desc)?;
        // if we fail to add a module the composer will tell us what is missing

        Ok(())
    }

    pub fn get(
        &mut self,
        render_device: &RenderDevice,
        pipeline: CachedPipelineId,
        resource: &ShaderResource,
        shader_defs: &[ShaderDefVal],
    ) -> Result<Arc<GpuShaderModule>, ShaderCacheError> {
        let id = resource.key();
        let shader = self
            .shaders
            .get(&id)
            .ok_or(ShaderCacheError::ShaderNotLoaded(resource.summary()))?;

        let data = self.data.entry(id).or_default();

        let n_asset_imports = shader
            .imports()
            .filter(|import| matches!(import, ShaderImport::AssetPath(_)))
            .count();
        let n_resolved_asset_imports = data
            .resolved_imports
            .keys()
            .filter(|import| matches!(import, ShaderImport::AssetPath(_)))
            .count();
        if n_asset_imports != n_resolved_asset_imports {
            return Err(ShaderCacheError::ShaderImportNotYetAvailable);
        }

        data.pipelines.insert(pipeline);

        let module = match data.processed_shaders.entry(shader_defs.into()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                debug!(
                    "processing shader {}, with shader defs {:?}",
                    id, shader_defs
                );
                let shader_source = match &shader.source {
                    _ => {
                        for import in shader.imports() {
                            Self::add_import_to_composer(
                                &mut self.composer,
                                &self.import_path_shaders,
                                &self.shaders,
                                import,
                            )?;
                        }

                        let shader_defs = shader_defs
                            .iter()
                            .chain(shader.shader_defs.iter())
                            .map(|def| match def.clone() {
                                ShaderDefVal::Bool(k, v) => {
                                    (k, naga_oil::compose::ShaderDefValue::Bool(v))
                                }
                                ShaderDefVal::Int(k, v) => {
                                    (k, naga_oil::compose::ShaderDefValue::Int(v))
                                }
                                ShaderDefVal::UInt(k, v) => {
                                    (k, naga_oil::compose::ShaderDefValue::UInt(v))
                                }
                            })
                            .collect::<std::collections::HashMap<_, _>>();

                        let naga = self.composer.make_naga_module(
                            naga_oil::compose::NagaModuleDescriptor {
                                shader_defs,
                                ..shader.into()
                            },
                        )?;

                        ShaderCacheSource::Naga(naga)
                    }
                };

                todo!()

                // let shader_module = render_device.create_shader_module(&ShaderModuleDescriptor {
                //     label: None,
                //     source: ShaderSource::N,
                // });

                // entry.insert(Arc::new(shader_module))
            }
        };

        todo!()
    }
}
