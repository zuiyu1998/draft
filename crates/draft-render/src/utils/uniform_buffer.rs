use crate::{Std140, frame_graph::BufferInfo, gfx_base::BufferUsages};

pub struct UniformBuffer {
    data: Vec<u8>,
    offset: u32,
    usage: BufferUsages,
    label: Option<String>,
}

impl Default for UniformBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl UniformBuffer {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            label: None,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_buffer_info(&self) -> BufferInfo {
        BufferInfo {
            label: self.label.as_deref().map(|s| s.to_string().into()),
            size: self.data.len() as u64,
            usage: self.usage,
            mapped_at_creation: false,
        }
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    pub fn with<T: Std140>(&mut self, value: &T) -> u32 {
        let mut size = 0;
        value.write(&mut self.data, &mut size);
        let offset = self.offset;

        self.offset += size;

        offset
    }
}
