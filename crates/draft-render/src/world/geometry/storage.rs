use crate::{
    Indices, Vertex,
    frame_graph::BufferInfo,
    gfx_base::{RawIndexFormat, RenderDevice, VertexBufferLayout},
};
use fyrox_core::log::Log;

use wgpu::{
    BufferUsages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    FrameworkError, Geometry, GeometryResource, TemporaryCache, render_resource::RenderBuffer,
};

pub struct GeometryData {
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
    pub layout: VertexBufferLayout,
}

pub struct IndexRenderBuffer {
    pub buffer: RenderBuffer,
    pub num_indices: u32,
    pub index_format: RawIndexFormat,
}

fn get_vertex_buffer_key(index: usize) -> String {
    format!("vertex_buffer_{}", index)
}

fn get_index_buffer_key(index: usize) -> String {
    format!("index_buffer_{}", index)
}

fn create_index_render_buffer(
    device: &RenderDevice,
    index: usize,
    indices: &Indices,
) -> IndexRenderBuffer {
    let bytes = indices.create_buffer();

    let buffer = device
        .wgpu_device()
        .create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &bytes,
            usage: BufferUsages::VERTEX,
        });

    let buffer_info = BufferInfo {
        label: None,
        size: bytes.len() as u64,
        mapped_at_creation: false,
        usage: BufferUsages::INDEX,
    };

    let key = get_index_buffer_key(index);

    let buffer = RenderBuffer {
        key,
        value: buffer,
        desc: buffer_info,
    };

    IndexRenderBuffer {
        buffer,
        num_indices: indices.len() as u32,
        index_format: indices.index_format(),
    }
}

fn create_vertex_render_buffer(
    device: &RenderDevice,
    index: usize,
    vertex: &Vertex,
) -> RenderBuffer {
    let bytes = vertex.create_buffer();

    let buffer = device
        .wgpu_device()
        .create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &bytes,
            usage: BufferUsages::VERTEX,
        });

    let buffer_info = BufferInfo {
        label: None,
        size: bytes.len() as u64,
        mapped_at_creation: false,
        usage: BufferUsages::VERTEX,
    };

    let key = get_vertex_buffer_key(index);

    RenderBuffer {
        key,
        value: buffer,
        desc: buffer_info,
    }
}

impl GeometryData {
    pub fn update(&mut self, geometry: &Geometry) {
        let index = geometry.cache_index.get();
        self.vertex_buffer.key = get_vertex_buffer_key(index);
    }

    pub fn new(device: &RenderDevice, geometry: &Geometry) -> Result<Self, FrameworkError> {
        let index = geometry.cache_index.get();
        let vertex_buffer = create_vertex_render_buffer(device, index, &geometry.vertex);

        let index_buffer = geometry
            .index
            .indices
            .as_ref()
            .map(|indices| create_index_render_buffer(device, index, indices));

        Ok(GeometryData {
            vertex_buffer,
            index_buffer,
            layout: geometry.vertex.get_vertex_layout(),
        })
    }
}

#[derive(Default)]
pub struct GeometryStorage {
    pub geometry_cache: TemporaryCache<GeometryData>,
}

impl GeometryStorage {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        geometry: &GeometryResource,
    ) -> Option<&GeometryData> {
        let mut geometry_state = geometry.state();

        if let Some(geometry_state) = geometry_state.data() {
            match self.geometry_cache.get_mut_or_insert_with(
                &geometry_state.cache_index,
                Default::default(),
                || GeometryData::new(device, geometry_state),
            ) {
                Ok(data) => {
                    data.update(geometry_state);
                    Some(data)
                }
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}
