use fyrox_core::{
    TypeUuidProvider, Uuid,
    reflect::*,
    uuid,
    visitor::{pod::PodVecView, *},
};
use fyrox_resource::ResourceData;

#[derive(Debug, Reflect, Clone)]
pub struct Image {
    pub data: Vec<u8>,
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
