use draft_window::{Window, WindowWrapper};
use winit::{event_loop::ActiveEventLoop, window::Window as WinitWindow};

pub fn create_window(event_loop: &ActiveEventLoop, _window: &Window) -> WindowWrapper {
    let winit_window_attributes = WinitWindow::default_attributes();

    let winit_window = event_loop.create_window(winit_window_attributes).unwrap();

    WindowWrapper::new(winit_window)
}
