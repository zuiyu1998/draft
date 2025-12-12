use draft_window::Window;
use winit::event_loop::ActiveEventLoop;

pub struct WinitWindows {}

impl WinitWindows {
    pub fn create_window(&mut self, _event_loop: &ActiveEventLoop, _window: &Window) {}
}
