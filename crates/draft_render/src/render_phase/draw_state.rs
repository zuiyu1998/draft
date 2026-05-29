use std::ops::Range;

use wgpu::IndexFormat;

use crate::{frame_graph::*, render_world::CachePipelineId};

pub struct DrawState {
    pipeline_id: Option<CachePipelineId>,
}

impl DrawState {
    pub fn is_pipeline_id_set(&self) -> bool {
        self.pipeline_id.is_some()
    }

    pub fn set_pipeline_id(&mut self, pipeline_id: CachePipelineId) {
        self.pipeline_id = Some(pipeline_id);
    }

    pub fn reset(&mut self) {
        self.pipeline_id = None;
    }
}

pub struct TrackedRenderPassBuilder<'a, 'b> {
    render_pass_builder: RenderPassBuilder<'a, 'b>,
    draw_state: DrawState,
}

impl PassNodeBuilderExt for TrackedRenderPassBuilder<'_, '_> {
    fn read_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceRead> {
        self.render_pass_builder.read_material(material)
    }

    fn write_material<M: ResourceMaterial>(
        &mut self,
        material: &M,
    ) -> ResourceRef<M::ResourceType, ResourceWrite> {
        self.render_pass_builder.write_material(material)
    }

    fn read<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceRead> {
        self.render_pass_builder.read(resource_handle)
    }

    fn write<ResourceType: TransientResource>(
        &mut self,
        resource_handle: ResourceHandle<ResourceType>,
    ) -> ResourceRef<ResourceType, ResourceWrite> {
        self.render_pass_builder.write(resource_handle)
    }

    fn read_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView {
        self.render_pass_builder.read_texture_handle(texture_handle)
    }

    fn write_texture_handle(
        &mut self,
        texture_handle: &TransientTextureViewHandle,
    ) -> TransientTextureView {
        self.render_pass_builder
            .write_texture_handle(texture_handle)
    }
}

impl<'a, 'b> TrackedRenderPassBuilder<'a, 'b> {
    pub fn new(render_pass_builder: RenderPassBuilder<'a, 'b>) -> Self {
        Self {
            render_pass_builder,
            draw_state: DrawState { pipeline_id: None },
        }
    }

    pub fn reset(&mut self) {
        self.draw_state.reset();
    }

    pub fn set_render_pipeline(&mut self, pipeline_id: CachePipelineId) {
        if self.draw_state.is_pipeline_id_set() {
            panic!("Render pipeline ID has already been set for this render pass builder.");
        }
        self.render_pass_builder.set_render_pipeline(pipeline_id);
        self.draw_state.set_pipeline_id(pipeline_id);
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &ResourceRef<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        self.render_pass_builder
            .set_vertex_buffer(slot, buffer_ref, offset, size);
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &ResourceRef<TransientBuffer, ResourceRead>,
        index_format: IndexFormat,
        offset: u64,
        size: u64,
    ) {
        self.render_pass_builder
            .set_index_buffer(buffer_ref, index_format, offset, size);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass_builder.draw(vertices, instances);
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass_builder
            .draw_indexed(indices, base_vertex, instances);
    }
}
