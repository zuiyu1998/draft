use std::{path::PathBuf, sync::Arc};

use fyrox_core::Uuid;
use fyrox_resource::{
    io::ResourceIo,
    loader::{BoxedLoaderFuture, ResourceLoader},
};

pub struct ImageLoader {}

impl ResourceLoader for ImageLoader {
    fn extensions(&self) -> &[&str] {
        todo!()
    }

    fn data_type_uuid(&self) -> Uuid {
        todo!()
    }

    fn load(&self, _path: PathBuf, _io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        todo!()
    }
}
