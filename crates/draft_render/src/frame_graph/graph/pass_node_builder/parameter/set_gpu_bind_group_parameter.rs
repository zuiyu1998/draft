use crate::frame_graph::{RenderPassCommand, RenderPassContext};

pub struct SetGpuBindGroupParameter {
    pub index: u32,
    pub bind_group: wgpu::BindGroup,
    pub offsets: Vec<u32>,
}

impl RenderPassCommand for SetGpuBindGroupParameter {
    fn execute(&self, render_pass_context: &mut RenderPassContext) {
        render_pass_context.set_gpu_bind_group(self.index, &self.bind_group, &self.offsets);
    }
}
