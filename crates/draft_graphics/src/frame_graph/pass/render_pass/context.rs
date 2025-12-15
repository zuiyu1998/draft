use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, TransientBindGroup, TransientBuffer},
    gfx_base::{CachedPipelineId, GpuRenderPass},
};

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

    pub fn set_render_pipeline(&mut self, id: CachedPipelineId) {
        let pipeline = self.pass_context.get_render_pipeline(id);
        self.render_pass
            .get_render_pass_mut()
            .set_pipeline(pipeline.wgpu());
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &TransientBindGroup, offsets: &[u32]) {
        let bind_group = bind_group.create_bind_group(self.pass_context);

        self.render_pass.get_render_pass_mut().set_bind_group(
            index,
            bind_group.get_wgpu_bind_group(),
            offsets,
        );
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);
        self.render_pass.get_render_pass_mut().set_vertex_buffer(
            slot,
            buffer
                .resource
                .get_wgpu_buffer()
                .slice(offset..(offset + size)),
        );
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);

        self.render_pass.get_render_pass_mut().set_index_buffer(
            buffer
                .resource
                .get_wgpu_buffer()
                .slice(offset..(offset + size)),
            index_format,
        );
    }
}
