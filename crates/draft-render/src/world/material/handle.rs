use std::ops::{Deref, DerefMut};

use fxhash::FxHashMap;
use fyrox_core::ImmutableString;
use wgpu::BufferUsages;

use crate::{
    BufferAllocator, BufferCache,
    frame_graph::{FrameGraph, Handle, ResourceMaterial, TransientBuffer},
    gfx_base::GpuSampler,
    render_resource::RenderTexture,
};

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
    pub sampler: GpuSampler,
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
                let buffer_key = buffer_cache.get_or_create(&buffer_info);

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
