use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, hash_map::Entry},
    sync::Arc,
};

use draft_graphics::frame_graph::gfx_base::{
    CachedPipelineId, GpuShaderModule, RenderDevice, ShaderModuleDescriptor, ShaderSource,
};
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
    waiting_on_import: HashMap<ShaderImport, Vec<ShaderId>>,
}

impl ShaderCache {
    pub fn new() -> Self {
        let composer = naga_oil::compose::Composer::default();

        Self {
            data: Default::default(),
            shaders: Default::default(),
            composer,
            import_path_shaders: Default::default(),
            waiting_on_import: Default::default(),
        }
    }

    fn clear(&mut self, id: ShaderId) -> Vec<CachedPipelineId> {
        let mut shaders_to_clear = vec![id];
        let mut pipelines_to_queue = Vec::new();
        while let Some(handle) = shaders_to_clear.pop() {
            if let Some(data) = self.data.get_mut(&handle) {
                data.processed_shaders.clear();
                pipelines_to_queue.extend(data.pipelines.iter().copied());
                shaders_to_clear.extend(data.dependents.iter().copied());

                if let Some(Shader { import_path, .. }) = self.shaders.get(&handle) {
                    self.composer
                        .remove_composable_module(&import_path.module_name());
                }
            }
        }

        pipelines_to_queue
    }

    pub fn remove(&mut self, shader: &ShaderResource) -> Vec<CachedPipelineId> {
        let id = shader.key();

        let pipelines_to_queue = self.clear(id);
        if let Some(shader) = self.shaders.remove(&id) {
            self.import_path_shaders.remove(shader.import_path());
        }

        pipelines_to_queue
    }

    pub fn set_shader(&mut self, shader: &ShaderResource) -> Vec<CachedPipelineId> {
        let id = shader.key();
        let shader = shader.data_ref().clone();

        let pipelines_to_queue = self.clear(id);
        let path = shader.import_path();
        self.import_path_shaders.insert(path.clone(), id);
        if let Some(waiting_shaders) = self.waiting_on_import.get_mut(path) {
            for waiting_shader in waiting_shaders.drain(..) {
                // resolve waiting shader import
                let data = self.data.entry(waiting_shader).or_default();
                data.resolved_imports.insert(path.clone(), id);
                // add waiting shader as dependent of this shader
                let data = self.data.entry(id).or_default();
                data.dependents.insert(waiting_shader);
            }
        }

        for import in shader.imports() {
            if let Some(import_id) = self.import_path_shaders.get(import).copied() {
                // resolve import because it is currently available
                let data = self.data.entry(id).or_default();
                data.resolved_imports.insert(import.clone(), import_id);
                // add this shader as a dependent of the import
                let data = self.data.entry(import_id).or_default();
                data.dependents.insert(id);
            } else {
                let waiting = self.waiting_on_import.entry(import.clone()).or_default();
                waiting.push(id);
            }
        }

        self.shaders.insert(id, shader);
        pipelines_to_queue
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
                let naga = match &shader.source {
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

                        naga
                    }
                };

                let shader_module = render_device.create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Naga(Cow::Owned(naga)),
                });

                entry.insert(Arc::new(shader_module))
            }
        };

        Ok(module.clone())
    }
}
