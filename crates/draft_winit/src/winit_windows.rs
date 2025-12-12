use bevy_platform::collections::HashMap;

use draft_window::{Window, WindowWrapper};
use fyrox_core::pool::Handle;
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowId},
};

pub struct WinitWindows {
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
    pub handle_to_winit: HashMap<Handle<Window>, WindowId>,
    pub winit_to_handle: HashMap<WindowId, Handle<Window>>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl WinitWindows {
    pub const fn new() -> Self {
        WinitWindows {
            windows: HashMap::new(),
            handle_to_winit: HashMap::new(),
            winit_to_handle: HashMap::new(),
            _not_send_sync: core::marker::PhantomData,
        }
    }

    pub fn request_redraw(&self) {
        for window in self.windows.values() {
            window.request_redraw();
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window: &Window,
        handle: Handle<Window>,
    ) -> &WindowWrapper<WinitWindow> {
        let winit_window_attributes = WinitWindow::default_attributes();

        let winit_window = event_loop.create_window(winit_window_attributes).unwrap();

        self.handle_to_winit.insert(handle, winit_window.id());
        self.winit_to_handle.insert(winit_window.id(), handle);

        self.windows
            .entry(winit_window.id())
            .insert(WindowWrapper::new(winit_window))
            .into_mut()
    }
}
