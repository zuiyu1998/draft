use core::{fmt::Debug, marker::PhantomData};

use crate::frame_graph::{
    AnyTransientResourceDescriptor, ArcTransientResource, IndexHandle, PassNode, TransientResource,
    TransientResourceDescriptor,
};

pub struct Ref<ResourceType: TransientResource, VieType> {
    pub raw: GraphRawResourceHandle,
    pub desc: <ResourceType as TransientResource>::Descriptor,
    _marker: PhantomData<(ResourceType, VieType)>,
}

impl<ResourceType: TransientResource, VieType> Debug for Ref<ResourceType, VieType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ref")
            .field("raw", &self.raw)
            .field("desc", &self.desc)
            .finish()
    }
}

impl<ResourceType: TransientResource, VieType> Ref<ResourceType, VieType> {
    pub fn new(
        raw: GraphRawResourceHandle,
        desc: <ResourceType as TransientResource>::Descriptor,
    ) -> Self {
        Self {
            raw,
            desc,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource, VieType> Clone for Ref<ResourceType, VieType> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            desc: self.desc.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource, VieType> PartialEq for Ref<ResourceType, VieType> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<ResourceType: TransientResource, VieType> Eq for Ref<ResourceType, VieType> {}

pub trait ResourceView {}

pub struct ResourceRead;

pub struct ResourceWrite;

impl ResourceView for ResourceRead {}

impl ResourceView for ResourceWrite {}

pub struct Handle<ResourceType: TransientResource> {
    pub raw: GraphRawResourceHandle,
    pub desc: <ResourceType as TransientResource>::Descriptor,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType: TransientResource> Clone for Handle<ResourceType> {
    fn clone(&self) -> Self {
        Handle {
            raw: self.raw.clone(),
            desc: self.desc.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource> Handle<ResourceType> {
    pub fn new(
        index: IndexHandle<ResourceNode>,
        version: u32,
        desc: <ResourceType as TransientResource>::Descriptor,
    ) -> Self {
        Self {
            raw: GraphRawResourceHandle { index, version },
            desc,
            _marker: PhantomData,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GraphRawResourceHandle {
    pub index: IndexHandle<ResourceNode>,
    pub version: u32,
}

pub struct ResourceNode {
    pub index: IndexHandle<ResourceNode>,
    pub name: String,
    pub first_use_pass: Option<IndexHandle<PassNode>>,
    pub last_user_pass: Option<IndexHandle<PassNode>>,
    version: u32,
    pub resource: VirtualResource,
}

pub struct ResourceRequese {
    pub index: IndexHandle<ResourceNode>,
    pub resource: VirtualResource,
}

pub struct ResourceRelease {
    pub index: IndexHandle<ResourceNode>,
}

#[derive(Clone)]
pub enum VirtualResource {
    Setuped(AnyTransientResourceDescriptor),
    Imported(ArcTransientResource),
}

impl VirtualResource {
    pub fn get_desc<ResourceType: TransientResource>(&self) -> ResourceType::Descriptor {
        let desc = match self {
            VirtualResource::Imported(resource) => resource.get_desc(),
            VirtualResource::Setuped(desc) => desc.clone(),
        };

        <ResourceType::Descriptor as TransientResourceDescriptor>::borrow_resource_descriptor(&desc)
            .clone()
    }
}

impl ResourceNode {
    pub fn new(name: &str, index: IndexHandle<ResourceNode>, resource: VirtualResource) -> Self {
        ResourceNode {
            name: name.to_string(),
            index,
            version: 0,
            first_use_pass: None,
            last_user_pass: None,
            resource,
        }
    }
}

impl ResourceNode {
    pub fn request(&self) -> ResourceRequese {
        ResourceRequese {
            index: self.index,
            resource: self.resource.clone(),
        }
    }

    pub fn get_handle<ResourceType: TransientResource>(&self) -> Handle<ResourceType> {
        let desc = self.get_desc::<ResourceType>().clone();
        Handle::new(self.index, self.version, desc)
    }

    pub fn get_desc<ResourceType: TransientResource>(&self) -> ResourceType::Descriptor {
        self.resource.get_desc::<ResourceType>()
    }

    pub fn release(&self) -> ResourceRelease {
        ResourceRelease { index: self.index }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1;
    }

    pub fn update_lifetime(&mut self, handle: IndexHandle<PassNode>) {
        if self.first_use_pass.is_none() {
            self.first_use_pass = Some(handle);
        }

        self.last_user_pass = Some(handle);
    }
}
