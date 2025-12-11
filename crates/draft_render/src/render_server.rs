use frame_graph::gfx_base::RenderDevice;

pub struct RenderServer {
    decice: RenderDevice,
}

impl RenderServer {
    pub fn decice(&self) -> &RenderDevice {
        &self.decice
    }
}
