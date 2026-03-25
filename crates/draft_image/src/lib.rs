mod loader;

use std::path::Path;

pub use loader::*;

use fyrox_core::{
    TypeUuidProvider, Uuid,
    io::FileError,
    reflect::*,
    uuid,
    visitor::{pod::PodVecView, *},
};
use fyrox_resource::{ResourceData, io::ResourceIo, options::ImportOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Reflect, Clone, Deserialize, Serialize)]
pub struct ImageImportOptions {}

impl ImportOptions for ImageImportOptions {}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FileLoadError(#[from] FileError),
}

#[derive(Debug, Reflect, Clone)]
pub struct Image {
    pub data: Vec<u8>,
}

pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
}

impl Image {
    pub fn load_from_memory(
        _data: &[u8],
        _import_options: ImageImportOptions,
    ) -> Result<Self, ImageError> {
        // let mut reader = image::ImageReader::new(std::io::Cursor::new(data));
        // reader.set_format(image_crate_format);
        // reader.no_limits();

        todo!()
    }

    pub(crate) async fn load_from_file<P: AsRef<Path>>(
        path: P,
        io: &dyn ResourceIo,
        import_options: ImageImportOptions,
    ) -> Result<Self, ImageError> {
        let data = io.load_file(path.as_ref()).await?;
        Self::load_from_memory(&data, import_options)
    }
}

impl Visit for Image {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut region = visitor.enter_region(name)?;

        let mut bytes_view = PodVecView::from_pod_vec(&mut self.data);
        bytes_view.visit("Data", &mut region)?;

        Ok(())
    }
}

impl TypeUuidProvider for Image {
    fn type_uuid() -> Uuid {
        uuid!("f41402e3-19d7-4209-b14f-e26603344e24")
    }
}

impl ResourceData for Image {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(
        &mut self,
        #[allow(unused_variables)] path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}
