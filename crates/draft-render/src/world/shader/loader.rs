use std::{path::PathBuf, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid};
use fyrox_resource::{
    io::ResourceIo,
    loader::{BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    state::LoadError,
};

use super::Shader;

pub struct ShaderLoader;

impl ResourceLoader for ShaderLoader {
    fn extensions(&self) -> &[&str] {
        &["wgsl", "vert", "frag"]
    }

    fn data_type_uuid(&self) -> Uuid {
        Shader::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        Box::pin(async move {
            let shader = Shader::from_file(&path, io.as_ref())
                .await
                .map_err(LoadError::new)?;
            Ok(LoaderPayload::new(shader))
        })
    }
}
