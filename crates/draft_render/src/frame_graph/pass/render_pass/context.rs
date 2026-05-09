use core::ops::Range;

use wgpu::IndexFormat;

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

    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.render_pass
            .get_render_pass_mut()
            .set_scissor_rect(x, y, width, height);
    }

    pub fn set_gpu_bind_group(
        &mut self,
        index: u32,
        bind_group: &wgpu::BindGroup,
        offsets: &[u32],
    ) {
        self.render_pass
            .get_render_pass_mut()
            .set_bind_group(index, Some(bind_group), offsets);
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

    pub fn set_render_pipeline(&mut self, pipeline_id: usize) {
        let pipeline = self
            .pass_context
            .pipeline_container
            .get_render_pipeline(pipeline_id)
            .expect("Render pipeline must have.");

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

        self.render_pass.get_render_pass_mut().set_index_buffer(
            buffer.resource.slice(offset..(offset + size)),
            index_format,
        );
    }
}
