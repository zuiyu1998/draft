use crate::frame_graph::{
    Ref, RenderPassCommand, RenderPassContext, ResourceRead, TransientBuffer,
};
use wgpu::IndexFormat;

pub struct SetIndexBufferParameter {
    pub buffer_ref: Ref<TransientBuffer, ResourceRead>,
    pub index_format: IndexFormat,
    pub offset: u64,
    pub size: u64,
}

impl RenderPassCommand for SetIndexBufferParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_index_buffer(
            &self.buffer_ref,
            self.index_format,
            self.offset,
            self.size,
        );
    }
}
