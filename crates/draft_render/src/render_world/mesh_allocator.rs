use std::collections::HashMap;

use draft_graphics::{Buffer, BufferInitDescriptor, BufferUsages, RenderDevice};
use draft_mesh::Mesh;
use wgpu::IndexFormat;

use crate::render_world::ResourceId;

pub struct MeshRenderData {
    vertex_buffer: Buffer,
    index_buffer: Option<IndexBufferRenderData>,
}

pub struct IndexBufferRenderData {
    pub buffer: Buffer,
    pub index_format: IndexFormat,
    pub num_indices: u32,
}

pub struct MeshAllocator {
    cache: HashMap<ResourceId<Mesh>, MeshRenderData>,
    device: RenderDevice,
}

impl MeshAllocator {
    pub fn new(device: &RenderDevice) -> Self {
        Self {
            cache: HashMap::new(),
            device: device.clone(),
        }
    }

    pub fn insert_mesh(&mut self, mesh_id: ResourceId<Mesh>, mesh: &Mesh) {
        let bytes = mesh.vertex_buffer.create_packed_vertex_buffer_data();

        let vertex_buffer = self.device.create_gpu_buffer_init(&BufferInitDescriptor {
            label: Some(&format!("Mesh Vertex Buffer {}", mesh_id.slot)),
            contents: &bytes,
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX,
        });

        let index_buffer = mesh.index_buffer.as_ref().map(|buffer| {
            let index_format = buffer.get_index_format();
            let num_indices = buffer.len() as u32;

            let bytes = buffer.create_packed_index_buffer_data();
            let buffer = self.device.create_gpu_buffer_init(&BufferInitDescriptor {
                label: Some(&format!("Mesh Index Buffer {}", mesh_id.slot)),
                contents: &bytes,
                usage: BufferUsages::COPY_DST | BufferUsages::INDEX,
            });

            IndexBufferRenderData {
                buffer,
                index_format,
                num_indices,
            }
        });

        self.cache.insert(
            mesh_id,
            MeshRenderData {
                vertex_buffer,
                index_buffer,
            },
        );
    }

    pub fn get_vertex_buffer(&self, mesh_id: ResourceId<Mesh>) -> &Buffer {
        &self.cache[&mesh_id].vertex_buffer
    }

    pub fn get_index_buffer(&self, mesh_id: ResourceId<Mesh>) -> Option<&IndexBufferRenderData> {
        self.cache[&mesh_id].index_buffer.as_ref()
    }
}
