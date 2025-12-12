use fyrox_core::pool::{Handle, Pool};

#[derive(Default, Clone)]
pub struct Window {}

#[derive(Default)]
pub struct Windows {
    primary: Option<Handle<Window>>,
    data: Pool<Window>,
}

impl Windows {
    pub fn spawn_primary(&mut self, window: Window) -> Handle<Window> {
        let handle = self.data.spawn(window);
        self.primary = Some(handle);

        handle
    }

    pub fn spawn(&mut self, window: Window) -> Handle<Window> {
        self.data.spawn(window)
    }

    pub fn remove(&mut self, handle: Handle<Window>) -> Window {
        self.data.free(handle)
    }
}
