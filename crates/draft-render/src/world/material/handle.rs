use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;
use wgpu::BufferUsages;

use crate::{
    BufferAllocator, BufferCache, FrameworkError, RenderWorld, ResourceKeyContainer,
    frame_graph::{FrameGraph, Handle, ResourceMaterial, TransientBuffer},
    gfx_base::RawSampler,
    render_resource::RenderTexture,
};

use super::{MaterialResourceBinding, ResourceBindings};

pub enum MaterialResourceHandle {
    Texture(MaterialTextureHandle),
    Sampler(MaterialSamplerHandle),
    PropertyGroup(MaterialPropertyGroupHandle),
    Buffer(MaterialBufferHandle),
}

pub struct MaterialBufferHandle {
    pub offset: u32,
    pub size: u32,
    pub handle: Handle<TransientBuffer>,
}

pub struct MaterialTextureHandle {
    pub texture: RenderTexture,
}

pub struct MaterialSamplerHandle {
    pub sampler: RawSampler,
}

pub struct MaterialPropertyGroupHandle {
    pub offset: u32,
    pub property_group_key: ImmutableString,
    pub size: u32,
}

#[derive(Default)]
pub struct MaterialBufferHandleCache(FxHashMap<ImmutableString, Handle<TransientBuffer>>);

impl MaterialBufferHandleCache {
    pub fn get_or_create(
        &mut self,
        buffer_allocator: &mut BufferAllocator,
        buffer_cache: &mut BufferCache,
        frame_graph: &mut FrameGraph,
        material_property_group_handle: &MaterialPropertyGroupHandle,
        usage: BufferUsages,
    ) -> Handle<TransientBuffer> {
        self.0
            .entry(material_property_group_handle.property_group_key.clone())
            .or_insert_with(|| {
                let buffer_info = buffer_allocator
                    .get_buffer_info(&material_property_group_handle.property_group_key, usage);
                let buffer_key = buffer_cache.get_or_create(buffer_info);

                buffer_cache.upload_bytes(
                    &buffer_key,
                    buffer_allocator.get_bytes(&material_property_group_handle.property_group_key),
                );

                let render_buffer = buffer_cache.get_render_buffer(&buffer_key);

                render_buffer.imported(frame_graph)
            })
            .clone()
    }
}

pub struct MaterialResourceHandleContainer(Vec<MaterialResourceHandle>);

impl Deref for MaterialResourceHandleContainer {
    type Target = Vec<MaterialResourceHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialResourceHandleContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MaterialResourceHandleContainer {
    pub fn extra(
        key_container: &ResourceKeyContainer,
        resource_bindings: &ResourceBindings,
        render_world: &mut RenderWorld,
    ) -> Result<Self, FrameworkError> {
        let mut target = vec![];

        for key in key_container.keys.iter() {
            match resource_bindings.get(key).unwrap() {
                MaterialResourceBinding::Texture(v) => {
                    let resource = v.value.clone().unwrap();
                    let texture_data = render_world.get_or_create_texture(&resource)?;

                    target.push(MaterialResourceHandle::Texture(MaterialTextureHandle {
                        texture: texture_data.render_data.texture.clone(),
                    }));

                    target.push(MaterialResourceHandle::Sampler(MaterialSamplerHandle {
                        sampler: texture_data.render_data.sampler.sampler().clone(),
                    }));
                }
                MaterialResourceBinding::PropertyGroup(v) => {
                    let named_values_container = v.get_named_values_container();
                    let (offset, size) = render_world
                        .buffer_allocator
                        .write(key, named_values_container);

                    target.push(MaterialResourceHandle::PropertyGroup(
                        MaterialPropertyGroupHandle {
                            offset,
                            property_group_key: key.clone(),
                            size,
                        },
                    ));
                }

                MaterialResourceBinding::BuiltIn => {
                    todo!()
                }
            }
        }

        Ok(MaterialResourceHandleContainer(target))
    }
}
