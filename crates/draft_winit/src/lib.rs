use draft_app::{App, AppInitializeParams, Plugin};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop};

#[derive(Default)]
pub struct WinitPlugin;

pub struct WinitAppRunnerState {
    app: App,
}

impl WinitAppRunnerState {
    pub fn new(app: App) -> Self {
        Self { app }
    }
}

impl ApplicationHandler for WinitAppRunnerState {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.initialize(AppInitializeParams {});
        self.app.finished();
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.destroy();
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.update();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            _ => {}
        }
    }
}

fn winit_runner(app: App, event_loop: EventLoop<()>) {
    let mut runner_state = WinitAppRunnerState::new(app);

    event_loop
        .run_app(&mut runner_state)
        .expect("run app faild.");
}

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        let event_loop = EventLoop::builder().build().expect("event loop faild.");

        app.set_runner(|app| winit_runner(app, event_loop));
    }
}
