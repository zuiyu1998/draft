use std::marker::PhantomData;

use draft_app::{App, AppError, AppExit};
use draft_render::GraphicsContext;

use log::error;
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::Message;

pub(crate) struct WinitAppRunnerState<T: Message> {
    app: App,
    app_exit: Option<AppExit>,
    _marker: PhantomData<T>,
}

fn initialize_graphics_context(
    app: &mut App,
    _event_loop: &ActiveEventLoop,
) -> Result<(), AppError> {
    let _params = match &app.graphics_context {
        GraphicsContext::Uninitialized(params) => params.clone(),
        _ => {
            return Err(AppError::Custom(
                "Graphics context is already initialized!".to_string(),
            ));
        }
    };

    Ok(())
}

impl<M: Message> ApplicationHandler<M> for WinitAppRunnerState<M> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        println!("new_events");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        initialize_graphics_context(&mut self.app, event_loop)
            .expect("Unable to initialize graphics context!");
    }

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

pub fn winit_runner<M: Message>(app: App, event_loop: EventLoop<M>) -> AppExit {
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
