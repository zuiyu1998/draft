use std::{path::PathBuf, sync::Arc};

use fyrox_core::{TypeUuidProvider, Uuid};
use fyrox_resource::{
    io::ResourceIo,
    loader::{BoxedImportOptionsLoaderFuture, BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    options::{BaseImportOptions, try_get_import_settings, try_get_import_settings_opaque},
    state::LoadError,
};

use crate::TextureImportOptions;

use super::Texture;

#[derive(Default)]
pub struct TextureLoader {
    pub default_import_options: TextureImportOptions,
}

impl ResourceLoader for TextureLoader {
    fn extensions(&self) -> &[&str] {
        &["gif", "jpeg", "png", "tga", "tiff", "bmp"]
    }

    fn data_type_uuid(&self) -> Uuid {
        Texture::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        let default_import_options = self.default_import_options.clone();
        Box::pin(async move {
            let io = io.as_ref();

            let import_options = try_get_import_settings(&path, io)
                .await
                .unwrap_or(default_import_options);

            let raw_texture = Texture::load_from_file(&path, io, import_options)
                .await
                .map_err(LoadError::new)?;

            Ok(LoaderPayload::new(raw_texture))
        })
    }

    fn try_load_import_settings(
        &self,
        resource_path: PathBuf,
        io: Arc<dyn ResourceIo>,
    ) -> BoxedImportOptionsLoaderFuture {
        Box::pin(async move {
            try_get_import_settings_opaque::<TextureImportOptions>(&resource_path, &*io).await
        })
    }

    fn default_import_options(&self) -> Option<Box<dyn BaseImportOptions>> {
        Some(Box::<TextureImportOptions>::default())
    }
}
