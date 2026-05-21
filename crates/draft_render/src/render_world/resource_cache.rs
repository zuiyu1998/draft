use std::{fmt::Debug, sync::Arc};

use draft_mesh::Mesh;
use draft_shader::Shader;
use fyrox_resource::{Resource, TypedResourceData, core::sparse::AtomicIndex};

use crate::{
    FrameworkError,
    render_world::{ResourceId, TemporaryCache},
};

pub struct RenderResourceData<R: Debug> {
    pub resource: Resource<R>,
    pub modifications_counter: u64,
}

pub trait RenderResource: TypedResourceData {
    fn get_cache_index(&self) -> Arc<AtomicIndex>;

    fn get_modifications_counter(&self) -> u64;
}

impl RenderResource for Shader {
    fn get_cache_index(&self) -> Arc<AtomicIndex> {
        self.cache_index.clone()
    }

    fn get_modifications_counter(&self) -> u64 {
        self.modifications_counter
    }
}

impl RenderResource for Mesh {
    fn get_cache_index(&self) -> Arc<AtomicIndex> {
        self.cache_index.clone()
    }

    fn get_modifications_counter(&self) -> u64 {
        let index_buffer_modifications_counter = self
            .index_buffer
            .as_ref()
            .map(|index_buffer| index_buffer.modifications_counter)
            .unwrap_or(0);

        self.vertex_buffer.modifications_counter + index_buffer_modifications_counter
    }
}

fn create_data<R: RenderResource>(
    resource: &Resource<R>,
    modifications_counter: u64,
) -> RenderResourceData<R> {
    RenderResourceData {
        resource: resource.clone(),
        modifications_counter,
    }
}

pub struct ResourceCache<R: RenderResource> {
    cache: TemporaryCache<RenderResourceData<R>>,
    update_resource_ids: Vec<ResourceId<R>>,
    add_resource_ids: Vec<ResourceId<R>>,
}

impl<R: RenderResource> Default for ResourceCache<R> {
    fn default() -> Self {
        ResourceCache {
            cache: TemporaryCache::default(),
            update_resource_ids: vec![],
            add_resource_ids: vec![],
        }
    }
}

impl<R: RenderResource> ResourceCache<R> {
    pub fn get_or_create_resource_id(
        &mut self,
        resource: &Resource<R>,
    ) -> Result<ResourceId<R>, FrameworkError> {
        if !resource.is_ok() {
            return Err(FrameworkError::ResourceNotLoaded(resource.summary()));
        }

        let (cache_index, modifications_counter) = {
            let resource = resource.data_ref();

            let cache_index = resource.get_cache_index();
            let modifications_counter = resource.get_modifications_counter();

            (cache_index, modifications_counter)
        };

        if let Some(entry) = self.cache.get_mut(&cache_index) {
            if entry.modifications_counter != modifications_counter {
                self.update_resource_ids
                    .push(ResourceId::new(cache_index.get()));
            }
        } else {
            let data = create_data(resource, modifications_counter);
            self.cache
                .spawn(data, cache_index.clone(), Default::default());

            self.add_resource_ids
                .push(ResourceId::new(cache_index.get()))
        }

        Ok(ResourceId::new(cache_index.get()))
    }

    pub fn get(&self, id: ResourceId<R>) -> Option<Resource<R>> {
        self.cache
            .buffer
            .get_raw(id.slot)
            .map(|etnry| etnry.resource.clone())
    }
}
