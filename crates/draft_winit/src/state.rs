use std::{marker::PhantomData, time::Instant};

use draft_app::{App, AppError, AppExit};
use draft_render::{
    GraphicsContext, InitializedGraphicsContext, WorldRenderer, initialize_render_server,
};

use draft_window::{RawHandleWrapper, Windows};
use log::error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{Message, WINIT_WINDOWS, winit_windows::WinitWindows};

pub const DEFAULT_UPDATE_RATE: f32 = 60.0;
/// Default time step (in seconds).
pub const DEFAULT_TIME_STEP: f32 = 1.0 / DEFAULT_UPDATE_RATE;

pub(crate) struct WinitAppRunnerState<T: Message> {
    app: App,
    app_exit: Option<AppExit>,
    lag: f32,
    fixed_time_step: f32,
    previous: Instant,
    _marker: PhantomData<T>,
}

impl<M: Message> WinitAppRunnerState<M> {
    fn destroy_graphics_context(&mut self) -> Result<(), AppError> {
        let graphics_context = match &self.app.graphics_context {
            GraphicsContext::Initialized(params) => params,
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already destroyed!".to_string(),
                ));
            }
        };
        let params = graphics_context.params.clone();
        self.app.graphics_context = GraphicsContext::Uninitialized(params);

        self.app.windows = Windows::default();

        WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
            *winit_windows = WinitWindows::new();
        });

        Ok(())
    }

    fn initialize_graphics_context(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<(), AppError> {
        let params = match &self.app.graphics_context {
            GraphicsContext::Uninitialized(params) => params.clone(),
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already initialized!".to_string(),
                ));
            }
        };

        let handle = self.app.windows.spawn_primary(params.window.clone());

        WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
            let winit_window = winit_windows.create_window(event_loop, &params.window, handle);

            let wrapper = RawHandleWrapper::new(winit_window).unwrap();

            let render_server = initialize_render_server(wrapper);

            self.app.graphics_context = GraphicsContext::Initialized(
                InitializedGraphicsContext::new(WorldRenderer::new(render_server), params),
            )
        });

        Ok(())
    }
}

impl<M: Message> ApplicationHandler<M> for WinitAppRunnerState<M> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);

        self.previous = Instant::now();
        self.initialize_graphics_context(event_loop)
            .expect("Unable to initialize graphics context!");
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.destroy_graphics_context()
            .expect("Unable to destroy graphics context!");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        game_loop_iteration(
            &mut self.app,
            &mut self.lag,
            &mut self.previous,
            self.fixed_time_step,
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                println!("render");
                self.app.render();
            }
            _ => {}
        }
    }
}

fn game_loop_iteration(app: &mut App, lag: &mut f32, previous: &mut Instant, fixed_time_step: f32) {
    let elapsed = previous.elapsed();
    *previous = Instant::now();
    *lag += elapsed.as_secs_f32();
}

impl<M: Message> WinitAppRunnerState<M> {
    fn new(app: App) -> Self {
        Self {
            app,
            app_exit: None,
            fixed_time_step: 0.0,
            _marker: PhantomData,
            lag: 0.0,
            previous: Instant::now(),
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
