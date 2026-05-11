use draft_graphics::{BufferInitDescriptor, RenderDevice};
use draft_mesh::{IndexBuffer, Mesh, MeshResource, VertexBuffer};
use wgpu::{Buffer, BufferUsages};

use crate::{
    FrameworkError,
    render_world::{ResourceId, TemporaryCache},
};

pub struct VertexBufferRenderData {
    pub modifications_count: u64,
    pub buffer: Buffer,
}

pub struct IndexBufferRenderData {
    pub modifications_count: u64,
    pub buffer: Buffer,
}

pub struct MeshRenderData {
    pub vertex_buffer: VertexBufferRenderData,
    pub index_buffer: Option<IndexBufferRenderData>,
}

#[derive(Default)]
pub struct MeshCache {
    cache: TemporaryCache<MeshRenderData>,
}

fn create_index_buffer_render_data(
    index_buffer: &IndexBuffer,
    device: &RenderDevice,
) -> Result<IndexBufferRenderData, FrameworkError> {
    let data = index_buffer.create_packed_index_buffer_data();

    let buffer = device.create_gpu_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: &data,
        usage: BufferUsages::INDEX,
    });

    Ok(IndexBufferRenderData {
        modifications_count: index_buffer.modifications_counter,
        buffer,
    })
}

fn create_vertex_buffer_render_data(
    vertex_buffer: &VertexBuffer,
    device: &RenderDevice,
) -> Result<VertexBufferRenderData, FrameworkError> {
    let data = vertex_buffer.create_packed_vertex_buffer_data();

    let buffer = device.create_gpu_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: &data,
        usage: BufferUsages::VERTEX,
    });

    Ok(VertexBufferRenderData {
        modifications_count: vertex_buffer.modifications_counter,
        buffer,
    })
}

fn create_mesh_render_data(
    mesh: &Mesh,
    device: &RenderDevice,
) -> Result<MeshRenderData, FrameworkError> {
    let vertex_buffer = create_vertex_buffer_render_data(&mesh.vertex_buffer, device)?;

    let index_buffer = match &mesh.index_buffer {
        Some(index_buffer) => Some(create_index_buffer_render_data(index_buffer, device)?),
        None => None,
    };

    Ok(MeshRenderData {
        vertex_buffer,
        index_buffer,
    })
}

impl MeshCache {
    pub fn get_create_mesh(
        &mut self,
        mesh: &MeshResource,
        device: &RenderDevice,
    ) -> Result<ResourceId<Mesh>, FrameworkError> {
        if !mesh.is_ok() {
            return Err(FrameworkError::MeshNotLoaded);
        }

        let mesh = mesh.data_ref();

        match self
            .cache
            .get_mut_or_insert_with(&mesh.cache_index, Default::default(), || {
                create_mesh_render_data(&mesh, device)
            }) {
            Ok(mesh_render_data) => {
                if mesh.vertex_buffer.modifications_counter
                    != mesh_render_data.vertex_buffer.modifications_count
                {
                    mesh_render_data.vertex_buffer =
                        create_vertex_buffer_render_data(&mesh.vertex_buffer, device)?;
                }

                if mesh.index_buffer.is_none() && mesh_render_data.index_buffer.is_some() {
                    mesh_render_data.index_buffer = None
                } else if mesh.index_buffer.is_some() && mesh_render_data.index_buffer.is_none() {
                    let index_buffer = match &mesh.index_buffer {
                        Some(index_buffer) => {
                            Some(create_index_buffer_render_data(index_buffer, device)?)
                        }
                        None => None,
                    };
                    mesh_render_data.index_buffer = index_buffer;
                } else if mesh.index_buffer.is_some() && mesh_render_data.index_buffer.is_some() {
                    let mesh_index_buffer = mesh.index_buffer.as_ref().unwrap();
                    let mesh_render_data_index_buffer =
                        mesh_render_data.index_buffer.as_mut().unwrap();

                    if mesh_index_buffer.modifications_counter
                        != mesh_render_data_index_buffer.modifications_count
                    {
                        *mesh_render_data_index_buffer =
                            create_index_buffer_render_data(mesh_index_buffer, device)?;
                    }
                }

                Ok(ResourceId::new(mesh.cache_index.get()))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
