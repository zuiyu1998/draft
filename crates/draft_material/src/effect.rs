use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use fyrox_core::{TypeUuidProvider, Uuid, io::FileError, reflect::*, uuid, visitor::*};
use fyrox_resource::{
    Resource, ResourceData,
    io::ResourceIo,
    loader::{BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    state::LoadError,
};
use serde::{Deserialize, Serialize};
use serde_pretty_yaml::Error as YamlError;
use thiserror::Error;

pub type MaterialEffectResource = Resource<MaterialEffect>;

use crate::MaterialBindGroup;


#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialTechnique {
    pub name: String,
    pub bind_groups: Vec<MaterialBindGroup>,
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize, Default)]
pub struct MaterialEffect {
    pub name: String,
    pub techniques: Vec<MaterialTechnique>,
    #[reflect(hidden)]
    modifications_counter: u64,
}

#[derive(Debug, Error)]
pub enum MaterialEffectError {
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error(transparent)]
    YamlError(#[from] YamlError),
}

impl MaterialEffect {
    pub fn modifications_counter(&self) -> u64 {
        self.modifications_counter
    }

    pub async fn from_file<P>(path: P, io: &dyn ResourceIo) -> Result<Self, MaterialEffectError>
    where
        P: AsRef<Path>,
    {
        let content = io.load_file(path.as_ref()).await?;
        let effect = serde_pretty_yaml::from_slice(&content)?;

        Ok(effect)
    }
}

impl TypeUuidProvider for MaterialEffect {
    fn type_uuid() -> Uuid {
        uuid!("d90ffdf8-b0b6-42f4-94ea-aef9ace1d628")
    }
}

impl ResourceData for MaterialEffect {
    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let value = serde_pretty_yaml::to_string(self)?;
        fs::write(path, value.as_bytes())?;

        Ok(())
    }

    fn can_be_saved(&self) -> bool {
        true
    }

    fn try_clone_box(&self) -> Option<Box<dyn ResourceData>> {
        Some(Box::new(self.clone()))
    }
}

#[derive(Default)]
pub struct MaterialEffectLoader;

impl ResourceLoader for MaterialEffectLoader {
    fn extensions(&self) -> &[&str] {
        &[".material_effect.yaml"]
    }

    fn data_type_uuid(&self) -> Uuid {
        <MaterialEffect as TypeUuidProvider>::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        Box::pin(async move {
            let material = MaterialEffect::from_file(&path, io.as_ref())
                .await
                .map_err(LoadError::new)?;
            Ok(LoaderPayload::new(material))
        })
    }
}
