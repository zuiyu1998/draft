use std::marker::PhantomData;

use draft_app::{
    App, AppError, AppExit, PluginsState,
    renderer::{GraphicsContext, InitializedGraphicsContext, WorldRenderer},
};
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
    event_loop: &ActiveEventLoop,
) -> Result<(), AppError> {
    if let GraphicsContext::Uninitialized(params) = &app.graphics_context {
        // let (window, server) = params.graphics_server_constructor.0(
        //     params,
        //     event_loop,
        //     params.window_attributes.clone(),
        //     params.named_objects,
        // )?;

        // let frame_size = (window.inner_size().width, window.inner_size().height);

        // let renderer = WorldRenderer::new(server, frame_size, &self.resource_manager)?;

        // app.graphics_context = GraphicsContext::Initialized(InitializedGraphicsContex::);

        Ok(())
    } else {
        Err(AppError::Custom(
            "Graphics context is already initialized!".to_string(),
        ))
    }
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
