use std::{num::NonZero, ops::Range};

use draft_graphics::{
    IndexFormat,
    frame_graph::{
        PassNodeBuilderExt, TransientBindGroup, TransientBindGroupBuffer, TransientBindGroupEntry,
        TransientBindGroupResource,
    },
    gfx_base::{BindGroupLayout, CachedPipelineId},
};
use draft_mesh::MeshResource;

use crate::{
    Buffer, BufferHandle, DrawError, RenderPhase, RenderPhaseContext, TrackedRenderPassBuilder,
};

pub struct RenderMeshInfo {
    pub key: u64,
    pub indice_info: RenderIndiceInfo,
}

impl RenderMeshInfo {
    pub fn from_mesh(mesh: &MeshResource) -> Self {
        let key = mesh.key();
        let mesh = mesh.data_ref();

        let indice_info = match mesh.indices() {
            None => RenderIndiceInfo::NonIndexed,
            Some(indices) => RenderIndiceInfo::Indexed {
                count: indices.len() as u32,
                index_format: indices.index_format(),
            },
        };

        RenderMeshInfo { key, indice_info }
    }

    pub fn render(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
        batch_range: &Range<u32>,
    ) -> Result<(), DrawError> {
        let Some(vertex_buffer_slice) = context.mesh_allocator.mesh_vertex_slice(&self.key) else {
            return Ok(());
        };

        builder.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));

        match &self.indice_info {
            RenderIndiceInfo::Indexed {
                count,
                index_format,
            } => {
                let Some(index_buffer_slice) = context.mesh_allocator.mesh_index_slice(&self.key)
                else {
                    return Ok(());
                };

                builder.set_index_buffer(*index_format, index_buffer_slice.buffer.slice(..));

                builder.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    batch_range.clone(),
                );
            }
            RenderIndiceInfo::NonIndexed => {
                builder.draw(vertex_buffer_slice.range, batch_range.clone());
            }
        }

        Ok(())
    }
}

pub enum RenderIndiceInfo {
    Indexed {
        count: u32,
        index_format: IndexFormat,
    },
    NonIndexed,
}

pub struct RenderBufferHandle {
    pub handle: BufferHandle,
    pub key: String,
    pub size: Option<NonZero<u64>>,
    pub offset: u64,
}

pub enum RenderTransientBindGroupResource {
    Buffer(RenderBufferHandle),
}

impl RenderTransientBindGroupResource {
    pub fn create_transient_bind_group_resource(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> TransientBindGroupResource {
        match self {
            RenderTransientBindGroupResource::Buffer(render_buffer) => {
                let desc = render_buffer.handle.desc.clone();
                let key = render_buffer.key.clone();

                let buffer = context.buffer_allocator.get_buffer(&render_buffer.handle);
                let buffer = Buffer::new(&key, buffer, desc);

                let buffer = builder.read_material(&buffer);

                TransientBindGroupResource::Buffer(TransientBindGroupBuffer {
                    buffer,
                    size: None,
                    offset: render_buffer.offset,
                })
            }
        }
    }
}

pub struct RenderTransientBindGroupEntry {
    pub binding: u32,
    pub resource: RenderTransientBindGroupResource,
}

impl RenderTransientBindGroupEntry {
    pub fn create_transient_bind_group_entry(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> TransientBindGroupEntry {
        TransientBindGroupEntry {
            binding: self.binding,
            resource: self
                .resource
                .create_transient_bind_group_resource(builder, context),
        }
    }
}

pub struct RenderTransientBindGroup {
    pub label: Option<String>,
    pub layout: BindGroupLayout,
    pub entries: Vec<RenderTransientBindGroupEntry>,
}

impl RenderTransientBindGroup {
    pub fn create_transient_bind_group(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> TransientBindGroup {
        let entries = self
            .entries
            .iter()
            .map(|entry| entry.create_transient_bind_group_entry(builder, context))
            .collect();

        TransientBindGroup {
            label: self.label.clone(),
            layout: self.layout.clone(),
            entries,
        }
    }
}

pub struct RenderBindGroup {
    pub index: usize,
    pub bind_group: RenderTransientBindGroup,
    pub offsets: Vec<u32>,
}

impl RenderBindGroup {
    pub fn render(&self, builder: &mut TrackedRenderPassBuilder, context: &RenderPhaseContext) {
        let bind_group = self
            .bind_group
            .create_transient_bind_group(builder, context);

        builder.set_bind_group(self.index, &bind_group, &self.offsets);
    }
}

pub struct BatchRenderMeshMaterial {
    pub pipeline_id: CachedPipelineId,
    pub bind_groups: Vec<RenderBindGroup>,
    pub mesh_info: RenderMeshInfo,
    pub batch_range: Range<u32>,
}

impl RenderPhase for BatchRenderMeshMaterial {
    fn render(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> Result<(), DrawError> {
        let Some(pipeline) = context
            .pipeline_container
            .get_render_pipeline(self.pipeline_id)
        else {
            return Ok(());
        };
        builder.set_render_pipeline(pipeline);

        for bind_group in self.bind_groups.iter() {
            bind_group.render(builder, context);
        }

        self.mesh_info.render(builder, context, &self.batch_range)?;

        Ok(())
    }
}
