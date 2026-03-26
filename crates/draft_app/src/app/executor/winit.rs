use crate::app::App;

use super::Executor;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub enum WinitUserEvent {
    /// Dummy event that just wakes up the winit event loop
    WakeUp,
    /// Tell winit that a window needs to be created
    WindowAdded,
}

#[derive(Default)]
pub struct WinitExecutor {
    pub run_on_any_thread: bool,
}

impl Executor for WinitExecutor {
    fn run(&mut self, app: App) {
        let mut event_loop_builder = EventLoop::<WinitUserEvent>::with_user_event();
        use winit::platform::windows::EventLoopBuilderExtWindows;
        event_loop_builder.with_any_thread(self.run_on_any_thread);

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        let mut runner_state = WinitAppRunnerState::new(app);
        if let Err(err) = event_loop.run_app(&mut runner_state) {
            println!("winit event loop returned an error: {err}");
        }
    }
}

pub(crate) struct WinitAppRunnerState {
    app: App,
}

impl WinitAppRunnerState {
    fn new(app: App) -> Self {
        Self { app }
    }
}

impl ApplicationHandler<WinitUserEvent> for WinitAppRunnerState {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {}

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app
            .initialize_graphics_context(event_loop)
            .expect("App initialize graphics context failed.");
        self.app.finish();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("app exit");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.app.render();
            }
            _ => (),
        }
    }
}

pub fn winit_runner(app: App, event_loop: EventLoop<WinitUserEvent>) {
    let mut runner_state = WinitAppRunnerState::new(app);
    if let Err(err) = event_loop.run_app(&mut runner_state) {
        println!("winit event loop returned an error: {err}");
    }
}
