use std::ops::Range;

use draft_graphics::{
    IndexFormat,
    frame_graph::{PassNodeBuilderExt, Ref, RenderPassBuilder, ResourceRead, TransientBuffer},
    gfx_base::{GpuRenderPipeline, PipelineContainer, RenderDevice},
};
use fyrox_core::err;

use crate::{BufferSlice, MeshAllocator};

#[derive(Default)]
pub struct DrawState {
    pipeline: Option<GpuRenderPipeline>,
    vertex_buffers: Vec<Option<(Ref<TransientBuffer, ResourceRead>, u64, u64)>>,
    index_buffer: Option<(Ref<TransientBuffer, ResourceRead>, u64, IndexFormat)>,

    stores_state: bool,
}

impl DrawState {
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
}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    state: DrawState,
}

impl<'a, 'b> TrackedRenderPassBuilder<'a, 'b> {
    pub fn new(render_pass_builder: RenderPassBuilder<'a, 'b>, device: &RenderDevice) -> Self {
        let limits = device.limits();
        let _max_bind_groups = limits.max_bind_groups as usize;
        let max_vertex_buffers = limits.max_vertex_buffers as usize;

        Self {
            render_pass_builder,
            state: DrawState {
                vertex_buffers: vec![None; max_vertex_buffers],
                ..Default::default()
            },
        }
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass_builder
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass_builder.draw(vertices, instances);
    }

    pub fn set_render_pipeline(&mut self, pipeline: &GpuRenderPipeline) {
        if self.state.is_pipeline_set() {
            err!("There are multiple rendering pipeline for the same drawcall.");
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
            buffer_slice.offset,
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
    fn render(&self, builder: &mut TrackedRenderPassBuilder, context: &RenderPhaseContext);
}
