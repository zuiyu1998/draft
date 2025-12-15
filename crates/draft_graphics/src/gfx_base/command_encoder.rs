use wgpu::CommandEncoderDescriptor as WgpuCommandEncoderDescriptor;

#[derive(Default)]
pub struct CommandEncoderDescriptor {
    pub label: Option<String>,
}

impl CommandEncoderDescriptor {
    pub fn get_buffer_desc(&self) -> WgpuCommandEncoderDescriptor<'_> {
        WgpuCommandEncoderDescriptor {
            label: self.label.as_deref(),
        }
    }
}
