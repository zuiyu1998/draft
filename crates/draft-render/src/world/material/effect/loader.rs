use fyrox_core::{TypeUuidProvider, Uuid};
use fyrox_resource::{
    io::ResourceIo,
    loader::{BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    state::LoadError,
};

use super::MaterialEffect;

pub struct MaterialEffectLoader;

impl ResourceLoader for MaterialEffectLoader {
    fn extensions(&self) -> &[&str] {
        &["material_effect"]
    }

    fn data_type_uuid(&self) -> Uuid {
        <MaterialEffect as TypeUuidProvider>::type_uuid()
    }

    fn load(
        &self,
        path: std::path::PathBuf,
        io: std::sync::Arc<dyn ResourceIo>,
    ) -> BoxedLoaderFuture {
        Box::pin(async move {
            let material = MaterialEffect::from_file(&path, io.as_ref())
                .await
                .map_err(LoadError::new)?;
            Ok(LoaderPayload::new(material))
        })
    }
}
