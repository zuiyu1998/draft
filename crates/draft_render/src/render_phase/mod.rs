mod mesh_material;

pub use mesh_material::*;
use thiserror::Error;

use std::ops::Range;

use draft_graphics::{
    IndexFormat,
    frame_graph::{
        Handle, PassNodeBuilderExt, Ref, RenderPassBuilder, ResourceMaterial, ResourceRead,
        ResourceWrite, TransientBindGroup, TransientBuffer, TransientResource,
    },
    gfx_base::{GpuRenderPipeline, PipelineContainer, RenderDevice},
};

use crate::{BufferAllocator, BufferSlice, MeshAllocator};

#[derive(Default)]
pub struct DrawState {
    pipeline: Option<GpuRenderPipeline>,
    vertex_buffers: Vec<Option<(Ref<TransientBuffer, ResourceRead>, u64, u64)>>,
    bind_groups: Vec<(Option<TransientBindGroup>, Vec<u32>)>,

    index_buffer: Option<(Ref<TransientBuffer, ResourceRead>, u64, IndexFormat)>,

    stores_state: bool,
}

impl DrawState {
    /// Marks the `bind_group` as bound to the `index`.
    fn set_bind_group(
        &mut self,
        index: usize,
        bind_group: &TransientBindGroup,
        dynamic_indices: &[u32],
    ) {
        let group = &mut self.bind_groups[index];
        group.0 = Some(bind_group.clone());
        group.1.clear();
        group.1.extend(dynamic_indices);
        self.stores_state = true;
    }

    /// Checks, whether the `bind_group` is already bound to the `index`.
    fn is_bind_group_set(
        &self,
        index: usize,
        bind_group: &TransientBindGroup,
        dynamic_indices: &[u32],
    ) -> bool {
        if let Some(current_bind_group) = self.bind_groups.get(index) {
            current_bind_group.0 == Some(bind_group.clone())
                && dynamic_indices == current_bind_group.1
        } else {
            false
        }
    }

    fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        index_format: IndexFormat,
    ) {
        self.index_buffer = Some((buffer_ref.clone(), offset, index_format));
        self.stores_state = true;
    }

    /// Checks, whether the index `buffer` is already bound.
    fn is_index_buffer_set(
        &self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        index_format: IndexFormat,
    ) -> bool {
        self.index_buffer == Some((buffer_ref.clone(), offset, index_format))
    }

    fn set_vertex_buffer(
        &mut self,
        index: usize,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        self.vertex_buffers[index] = Some((buffer_ref.clone(), offset, size));
        self.stores_state = true;
    }

    fn is_vertex_buffer_set(
        &self,
        index: usize,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) -> bool {
        if let Some(current) = self.vertex_buffers.get(index) {
            *current == Some((buffer_ref.clone(), offset, size))
        } else {
            false
        }
    }

    pub fn is_pipeline_set(&self) -> bool {
        self.pipeline.is_some()
    }

    pub fn set_render_pipeline(&mut self, pipeline: &GpuRenderPipeline) {
        self.pipeline = Some(pipeline.clone())
    }
}

pub struct RenderPhaseContext<'a> {
    pub pipeline_container: &'a PipelineContainer,
    pub mesh_allocator: &'a MeshAllocator,
    pub buffer_allocator: &'a BufferAllocator,
}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    state: DrawState,
}

impl<'a, 'b> PassNodeBuilderExt for TrackedRenderPassBuilder<'a, 'b> {
    fn read_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> Ref<M::ResourceType, ResourceRead> {
        self.render_pass_builder.read_material(material)
    }

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> Ref<M::ResourceType, ResourceWrite> {
        self.render_pass_builder.write_material(material)
    }

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceRead> {
        self.render_pass_builder.read(resource_handle)
    }

    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: Handle<ResourceType>,
    ) -> Ref<ResourceType, ResourceWrite> {
        self.render_pass_builder.write(resource_handle)
    }
}

impl<'a, 'b> TrackedRenderPassBuilder<'a, 'b> {
    pub fn new(render_pass_builder: RenderPassBuilder<'a, 'b>, device: &RenderDevice) -> Self {
        let limits = device.limits();
        let max_bind_groups = limits.max_bind_groups as usize;
        let max_vertex_buffers = limits.max_vertex_buffers as usize;

        Self {
            render_pass_builder,
            state: DrawState {
                vertex_buffers: vec![None; max_vertex_buffers],
                bind_groups: vec![(None, Vec::new()); max_bind_groups],
                ..Default::default()
            },
        }
    }

    pub fn set_bind_group(
        &mut self,
        index: usize,
        bind_group: &TransientBindGroup,
        offsets: &[u32],
    ) {
        if self.state.is_bind_group_set(index, bind_group, offsets) {
            #[cfg(feature = "detailed_trace")]
            trace!(
                "set bind_group {} (already set): {:?} ({:?})",
                index, bind_group, offsets
            );
            return;
        }

        self.render_pass_builder
            .set_bind_group(index as u32, bind_group, offsets);

        self.state.set_bind_group(index, bind_group, offsets);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass_builder
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass_builder.draw(vertices, instances);
    }

    pub fn set_render_pipeline(&mut self, pipeline: &GpuRenderPipeline) {
        #[cfg(feature = "detailed_trace")]
        trace!("set pipeline: {:?}", pipeline);
        if self.state.is_pipeline_set() {
            return;
        }

        self.render_pass_builder.set_render_pipeline(pipeline);
        self.state.set_render_pipeline(pipeline);
    }

    pub fn set_index_buffer(&mut self, index_format: IndexFormat, buffer_slice: BufferSlice<'_>) {
        let buffer_ref = self.render_pass_builder.read_material(buffer_slice.buffer);

        if self
            .state
            .is_index_buffer_set(&buffer_ref, buffer_slice.offset, index_format)
        {
            return;
        }

        self.render_pass_builder.set_index_buffer(
            &buffer_ref,
            index_format,
            buffer_slice.offset,
            buffer_slice.size,
        );
        self.state
            .set_index_buffer(&buffer_ref, buffer_slice.offset, index_format);
    }

    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'_>) {
        let buffer_ref = self.render_pass_builder.read_material(buffer_slice.buffer);

        if self.state.is_vertex_buffer_set(
            slot as usize,
            &buffer_ref,
            buffer_slice.offset,
            buffer_slice.size,
        ) {
            return;
        }

        self.render_pass_builder.set_vertex_buffer(
            slot,
            &buffer_ref,
            buffer_slice.offset,
            buffer_slice.size,
        );

        self.state.set_vertex_buffer(
            slot as usize,
            &buffer_ref,
            buffer_slice.offset,
            buffer_slice.size,
        );
    }
}

pub trait RenderPhase: 'static {
    fn render(
        &self,
        builder: &mut TrackedRenderPassBuilder,
        context: &RenderPhaseContext,
    ) -> Result<(), DrawError>;
}

#[derive(Debug, Error)]
pub enum DrawError {
    #[error("Failed to execute render command {0:?}")]
    RenderCommandFailure(&'static str),
}
