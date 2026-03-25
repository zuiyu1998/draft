use std::ops::Range;

use wgpu::{IndexFormat, RenderPipeline};

use crate::frame_graph::{GpuRenderPass, PassContext, ResourceRead, ResourceRef, TransientBuffer};

pub struct RenderPassContext<'a, 'b> {
    render_pass: GpuRenderPass,
    pass_context: &'b mut PassContext<'a>,
}

impl<'a, 'b> RenderPassContext<'a, 'b> {
    pub fn new(render_pass: GpuRenderPass, pass_context: &'b mut PassContext<'a>) -> Self {
        RenderPassContext {
            render_pass,
            pass_context,
        }
    }

    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass
            .get_render_pass_mut()
            .draw_indexed(indices, base_vertex, instances);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass
            .get_render_pass_mut()
            .draw(vertices, instances);
    }

    pub fn set_render_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.render_pass
            .get_render_pass_mut()
            .set_pipeline(pipeline);
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &ResourceRef<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);
        self.render_pass
            .get_render_pass_mut()
            .set_vertex_buffer(slot, buffer.resource.slice(offset..(offset + size)));
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &ResourceRef<TransientBuffer, ResourceRead>,
        index_format: IndexFormat,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);

        self.render_pass
            .get_render_pass_mut()
            .set_index_buffer(buffer.resource.slice(offset..(offset + size)), index_format);
    }
}
