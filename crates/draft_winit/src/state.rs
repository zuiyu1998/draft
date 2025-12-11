use std::marker::PhantomData;

use draft_app::{App, AppExit, PluginsState};
use log::error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::Message;

pub(crate) struct WinitAppRunnerState<T: Message> {
    app: App,
    app_exit: Option<AppExit>,
    _marker: PhantomData<T>,
}

impl<M: Message> ApplicationHandler<M> for WinitAppRunnerState<M> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl<M: Message> WinitAppRunnerState<M> {
    fn new(app: App) -> Self {
        Self {
            app,
            app_exit: None,
            _marker: PhantomData,
        }
    }
}

pub fn winit_runner<M: Message>(mut app: App, event_loop: EventLoop<M>) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let mut runner_state = WinitAppRunnerState::new(app);

    if let Err(err) = event_loop.run_app(&mut runner_state) {
        error!("winit event loop returned an error: {err}");
    }
    // If everything is working correctly then the event loop only exits after it's sent an exit code.
    runner_state.app_exit.unwrap_or_else(|| {
        error!("Failed to receive an app exit code! This is a bug");
        AppExit::error()
    })
}
