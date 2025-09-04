use std::sync::Arc;

use crate::{
    Indices, Vertex,
    frame_graph::{BufferInfo, TransientBuffer},
    render_resource::RenderBuffer,
};
use draft_gfx_base::{BufferInitDescriptor, RawIndexFormat, RenderDevice, VertexBufferLayout};

use fyrox_core::sparse::AtomicIndex;
use wgpu::BufferUsages;

use crate::{FrameworkError, Geometry, GeometryResource, TemporaryCache};

pub struct GeometryData {
    pub layout: VertexBufferLayout,
    vertex_buffer: TransientBuffer,
    index_buffer: Option<IndexBuffer>,
    cache_index: Arc<AtomicIndex>,
}

impl GeometryData {
    pub fn get_vertex_buffer(&self) -> RenderBuffer {
        RenderBuffer {
            key: get_vertex_buffer_key(self.cache_index.get()),
            value: self.vertex_buffer.resource.clone(),
            desc: self.vertex_buffer.desc.clone(),
        }
    }

    pub fn get_index_buffer(&self) -> Option<IndexRenderBuffer> {
        self.index_buffer
            .as_ref()
            .map(|index_buffer| IndexRenderBuffer {
                num_indices: index_buffer.num_indices,
                index_format: index_buffer.index_format,
                buffer: RenderBuffer {
                    key: get_index_buffer_key(self.cache_index.get()),
                    value: index_buffer.buffer.resource.clone(),
                    desc: index_buffer.buffer.desc.clone(),
                },
            })
    }
}

#[derive(Clone)]
pub struct IndexBuffer {
    pub buffer: TransientBuffer,
    pub num_indices: u32,
    pub index_format: RawIndexFormat,
}

#[derive(Clone)]
pub struct IndexRenderBuffer {
    pub buffer: RenderBuffer,
    pub num_indices: u32,
    pub index_format: RawIndexFormat,
}

fn get_vertex_buffer_key(index: usize) -> String {
    format!("vertex_buffer_{index}")
}

fn get_index_buffer_key(index: usize) -> String {
    format!("index_buffer_{index}")
}

fn create_index_render_buffer(device: &RenderDevice, indices: &Indices) -> IndexBuffer {
    let bytes = indices.create_buffer();

    let init_desc = BufferInitDescriptor {
        label: None,
        contents: &bytes,
        usage: BufferUsages::INDEX,
    };

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: &bytes,
        usage: BufferUsages::INDEX,
    });

    let buffer_desc = init_desc.to_buffer_desc();

    let buffer = TransientBuffer {
        resource: buffer,
        desc: BufferInfo::from_buffer_desc(&buffer_desc),
    };

    IndexBuffer {
        buffer,
        num_indices: indices.len() as u32,
        index_format: indices.index_format(),
    }
}

fn create_vertex_render_buffer(device: &RenderDevice, vertex: &Vertex) -> TransientBuffer {
    let bytes = vertex.create_buffer();

    let init_desc = BufferInitDescriptor {
        label: None,
        contents: &bytes,
        usage: BufferUsages::VERTEX,
    };

    let buffer = device.create_buffer_init(&init_desc);
    let buffer_desc = init_desc.to_buffer_desc();

    TransientBuffer {
        resource: buffer,
        desc: BufferInfo::from_buffer_desc(&buffer_desc),
    }
}

impl GeometryData {
    pub fn new(device: &RenderDevice, geometry: &Geometry) -> Result<Self, FrameworkError> {
        let vertex_buffer = create_vertex_render_buffer(device, &geometry.vertex);

        let index_buffer = geometry
            .index
            .indices
            .as_ref()
            .map(|indices| create_index_render_buffer(device, indices));

        Ok(GeometryData {
            vertex_buffer,
            index_buffer,
            layout: geometry.vertex.get_vertex_layout(),
            cache_index: geometry.cache_index.clone(),
        })
    }
}

#[derive(Default)]
pub struct GeometryCache {
    pub geometry_cache: TemporaryCache<GeometryData>,
}

impl GeometryCache {
    pub fn get_or_create(
        &mut self,
        device: &RenderDevice,
        geometry: &GeometryResource,
    ) -> Result<&GeometryData, FrameworkError> {
        let mut geometry_state = geometry.state();

        if let Some(geometry_state) = geometry_state.data() {
            match self.geometry_cache.get_or_insert_with(
                &geometry_state.cache_index,
                Default::default(),
                || GeometryData::new(device, geometry_state),
            ) {
                Ok(data) => Ok(data),
                Err(error) => Err(error),
            }
        } else {
            Err(geometry.clone().into())
        }
    }
}
