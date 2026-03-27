use std::sync::Arc;

use draft_core::{RenderResource, ResourceId};
use fyrox_core::{TypeUuidProvider, Uuid, reflect::*, sparse::AtomicIndex, uuid, visitor::*};
use fyrox_resource::ResourceData;

#[derive(Debug, Clone, Reflect)]
pub struct Pipeline {
    #[reflect(hidden)]
    pub cache_index: Arc<AtomicIndex>,
}

impl RenderResource for Pipeline {
    fn get_resource_id(&self) -> ResourceId {
        self.cache_index.clone().into()
    }
}

impl TypeUuidProvider for Pipeline {
    fn type_uuid() -> Uuid {
        uuid!("e1ce1983-4e80-4d8b-a4e5-9b05112e3b5c")
    }
}

impl Visit for Pipeline {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut _region = visitor.enter_region(name)?;

        Ok(())
    }
}

impl ResourceData for Pipeline {
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
