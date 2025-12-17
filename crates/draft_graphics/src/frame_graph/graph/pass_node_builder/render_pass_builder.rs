use std::mem::take;

use crate::{
    frame_graph::{
        Ref, RenderPass, ResourceRead, TransientBindGroup, TransientBuffer,
        TransientRenderPassColorAttachment,
    },
    gfx_base::CachedPipelineId,
};

use super::{PassBuilder, RenderPassExt};

pub struct RenderPassBuilder<'a, 'b> {
    render_pass: RenderPass,
    pass_builder: &'b mut PassBuilder<'a>,
}

impl Drop for RenderPassBuilder<'_, '_> {
    fn drop(&mut self) {
        self.finish();
    }
}

impl<'a, 'b> RenderPassBuilder<'a, 'b> {
    pub fn new(pass_builder: &'b mut PassBuilder<'a>, name: &str) -> Self {
        let mut render_pass = RenderPass::default();
        render_pass.set_pass_name(name);

        Self {
            render_pass,
            pass_builder,
        }
    }

    pub fn add_color_attachment(
        &mut self,
        color_attachment: TransientRenderPassColorAttachment,
    ) -> &mut Self {
        self.render_pass
            .add_color_attachment(Some(color_attachment));
        self
    }

    pub fn set_render_pipeline(&mut self, id: CachedPipelineId) -> &mut Self {
        self.render_pass.set_render_pipeline(id);
        self
    }

    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &TransientBindGroup,
        offsets: &[u32],
    ) -> &mut Self {
        self.render_pass.set_bind_group(index, bind_group, offsets);

        self
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) -> &mut Self {
        self.render_pass
            .set_index_buffer(buffer_ref, index_format, offset, size);

        self
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) -> &mut Self {
        self.render_pass
            .set_vertex_buffer(slot, buffer_ref, offset, size);
        self
    }

    pub fn create_render_pass_builder(&mut self) -> &mut Self {
        self.finish();

        self
    }

    fn finish(&mut self) {
        let render_pass = take(&mut self.render_pass);
        self.pass_builder.push(render_pass);
    }
}
