use fyrox_core::{TypeUuidProvider, Uuid, io::FileError, reflect::*, uuid, visitor::*};
use fyrox_resource::{
    Resource, ResourceData,
    io::ResourceIo,
    loader::{BoxedImportOptionsLoaderFuture, BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    options::{BaseImportOptions, ImportOptions, try_get_import_settings_opaque},
    state::LoadError,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    path::{Path, PathBuf},
    string::FromUtf8Error,
    sync::Arc,
};
use thiserror::Error;

pub type ShaderResource = Resource<Shader>;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Reflect, Visit, Default)]
pub enum ShaderStage {
    #[default]
    Vertex,
    Fragment,
    Compute,
    Task,
    Mesh,
}

#[derive(Debug, Clone, Reflect, Visit)]
pub enum Source {
    Wgsl(String),
    Glsl(String, ShaderStage),
}

impl Default for Source {
    fn default() -> Self {
        Self::Wgsl("".to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Reflect, Visit)]
pub enum ShaderImport {
    AssetPath(String),
    Custom(String),
}

impl Default for ShaderImport {
    fn default() -> Self {
        Self::AssetPath("".to_string())
    }
}

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct Shader {
    pub path: String,
    pub source: Source,
    pub import_path: ShaderImport,
    pub imports: Vec<ShaderImport>,
    // any shader defs that will be included when this module is used
    pub shader_defs: Vec<ShaderDefVal>,
}

impl Shader {
    fn preprocess(source: &str, path: &str) -> (ShaderImport, Vec<ShaderImport>) {
        let (import_path, imports, _) = naga_oil::compose::get_preprocessor_data(source);

        let import_path = import_path
            .map(ShaderImport::Custom)
            .unwrap_or_else(|| ShaderImport::AssetPath(path.to_owned()));

        let imports = imports
            .into_iter()
            .map(|import| {
                if import.import.starts_with('\"') {
                    let import = import
                        .import
                        .chars()
                        .skip(1)
                        .take_while(|c| *c != '\"')
                        .collect();
                    ShaderImport::AssetPath(import)
                } else {
                    ShaderImport::Custom(import.import)
                }
            })
            .collect();

        (import_path, imports)
    }

    pub fn from_wgsl(source: String, path: impl Into<String>) -> Shader {
        let path = path.into();
        let (import_path, imports) = Shader::preprocess(&source, &path);
        Shader {
            path,
            imports,
            import_path,
            source: Source::Wgsl(source),
            shader_defs: Default::default(),
        }
    }

    pub fn from_wgsl_with_defs(
        source: String,
        path: impl Into<String>,
        import_options: ShaderImportOptions,
    ) -> Shader {
        Self {
            shader_defs: import_options.shader_defs,
            ..Self::from_wgsl(source, path)
        }
    }

    pub async fn from_file<P>(
        path: P,
        io: &dyn ResourceIo,
        import_options: ShaderImportOptions,
    ) -> Result<Self, ShaderError>
    where
        P: AsRef<Path>,
    {
        let content = io.load_file(path.as_ref()).await?;
        let ext = path.as_ref().extension().unwrap().to_str().unwrap();
        let resource_path = path.as_ref().to_str().unwrap().to_string();

        // On windows, the path will inconsistently use \ or /.
        // TODO: remove this once AssetPath forces cross-platform "slash" consistency. See #10511
        let resource_path = resource_path.replace(std::path::MAIN_SEPARATOR, "/");

        let shader = match ext {
            "wgsl" => Shader::from_wgsl_with_defs(
                String::from_utf8(content)?,
                resource_path,
                import_options,
            ),
            _ => panic!("unhandled extension: {ext}"),
        };

        Ok(shader)
    }
}

impl TypeUuidProvider for Shader {
    fn type_uuid() -> Uuid {
        uuid!("639a3c19-5c7e-4692-9917-e9bf7f6d706e")
    }
}

impl ResourceData for Shader {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, _path: &Path) -> Result<(), Box<dyn Error>> {
        //todo
        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Deserialize, Serialize, Reflect, Visit)]
pub enum ShaderDefVal {
    Bool(String, bool),
    Int(String, i32),
    UInt(String, u32),
}

impl Default for ShaderDefVal {
    fn default() -> Self {
        Self::Bool("".to_string(), false)
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, Reflect)]
pub struct ShaderImportOptions {
    pub shader_defs: Vec<ShaderDefVal>,
}

impl ImportOptions for ShaderImportOptions {}

#[derive(Default)]
pub struct ShaderLoader {
    default_import_options: ShaderImportOptions,
}

impl ResourceLoader for ShaderLoader {
    fn extensions(&self) -> &[&str] {
        &["wgsl", "vert", "frag"]
    }

    fn data_type_uuid(&self) -> Uuid {
        <Shader as TypeUuidProvider>::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        let default_import_options = self.default_import_options.clone();

        Box::pin(async move {
            let material = Shader::from_file(&path, io.as_ref(), default_import_options)
                .await
                .map_err(LoadError::new)?;
            Ok(LoaderPayload::new(material))
        })
    }

    fn try_load_import_settings(
        &self,
        resource_path: PathBuf,
        io: Arc<dyn ResourceIo>,
    ) -> BoxedImportOptionsLoaderFuture {
        Box::pin(async move {
            try_get_import_settings_opaque::<ShaderImportOptions>(&resource_path, &*io).await
        })
    }

    fn default_import_options(&self) -> Option<Box<dyn BaseImportOptions>> {
        Some(Box::<ShaderImportOptions>::default())
    }
}
