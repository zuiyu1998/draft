use std::collections::HashMap;

use crate::frame_graph::{
    AnyTransientResource, ArcTransientResource, IndexHandle, Ref, ResourceNode, ResourceRelease,
    ResourceRequese, ResourceView, TransientResource, TransientResourceCache,
    TransientResourceCreator, VirtualResource,
};
use crate::gfx_base::RenderDevice;

#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<IndexHandle<ResourceNode>, AnyTransientResource>,
}

impl ResourceTable {
    pub fn get_resource<ResourceType: TransientResource, ViewType: ResourceView>(
        &self,
        resource_ref: &Ref<ResourceType, ViewType>,
    ) -> &ResourceType {
        self.resources
            .get(&resource_ref.raw.index)
            .map(|res| TransientResource::borrow_resource(res))
            .expect("must have resource")
    }

    pub fn request_resource(
        &mut self,
        request: &ResourceRequese,
        device: &RenderDevice,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        let index = request.index;
        let resource = match &request.resource {
            VirtualResource::Imported(resource) => match &resource {
                ArcTransientResource::Texture(resource) => {
                    AnyTransientResource::ImportedTexture(resource.clone())
                }
                ArcTransientResource::Buffer(resource) => {
                    AnyTransientResource::ImportedBuffer(resource.clone())
                }
            },
            VirtualResource::Setuped(desc) => transient_resource_cache
                .get_resource(desc)
                .unwrap_or_else(|| device.create_resource(desc)),
        };

        self.resources.insert(index, resource);
    }

    pub fn release_resource(
        &mut self,
        release: &ResourceRelease,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        if let Some(resource) = self.resources.remove(&release.index) {
            match resource {
                AnyTransientResource::OwnedBuffer(buffer) => {
                    transient_resource_cache.insert_resource(
                        buffer.desc.clone().into(),
                        AnyTransientResource::OwnedBuffer(buffer),
                    );
                }
                AnyTransientResource::OwnedTexture(texture) => {
                    transient_resource_cache.insert_resource(
                        texture.desc.clone().into(),
                        AnyTransientResource::OwnedTexture(texture),
                    );
                }
                _ => {}
            }
        }
    }
}
