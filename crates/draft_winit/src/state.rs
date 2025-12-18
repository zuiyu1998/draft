use std::{marker::PhantomData, time::Instant};

use draft_app::{App, AppExit, IEventLoop};

use draft_window::{Window, WindowWrapper};
use tracing::error;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowId},
};

use crate::Message;

pub const DEFAULT_UPDATE_RATE: f32 = 60.0;
/// Default time step (in seconds).
pub const DEFAULT_TIME_STEP: f32 = 1.0 / DEFAULT_UPDATE_RATE;

pub struct ActiveEventLoopRef<'a> {
    event_loop: &'a ActiveEventLoop,
}

impl<'a> ActiveEventLoopRef<'a> {
    pub fn new(event_loop: &'a ActiveEventLoop) -> Self {
        Self { event_loop }
    }
}

impl<'a> IEventLoop for ActiveEventLoopRef<'a> {
    fn create_window(&self, _window: &Window) -> WindowWrapper {
        let winit_window_attributes = WinitWindow::default_attributes();

        let winit_window = self
            .event_loop
            .create_window(winit_window_attributes)
            .unwrap();

        WindowWrapper::new(winit_window)
    }
}

pub(crate) struct WinitAppRunnerState<T: Message> {
    app: App,
    app_exit: Option<AppExit>,
    lag: f32,
    desired_update_rate: f32,
    previous: Instant,
    last_throttle_frame_number: usize,
    throttle_threshold: f32,
    throttle_frame_interval: usize,
    _marker: PhantomData<T>,
}

impl<M: Message> ApplicationHandler<M> for WinitAppRunnerState<M> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);

        self.previous = Instant::now();

        let event_loop_ref = ActiveEventLoopRef::new(event_loop);

        self.app
            .initialize_graphics_context(&event_loop_ref)
            .expect("Unable to initialize graphics context!");
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.app
            .destroy_graphics_context()
            .expect("Unable to destroy graphics context!");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let frame_count = self.app.frame_count;
        let fixed_time_step = 1.0 / self.desired_update_rate;

        game_loop_iteration(
            &mut self.app,
            &mut self.previous,
            &mut self.lag,
            fixed_time_step,
            self.throttle_threshold,
            self.throttle_frame_interval,
            frame_count,
            &mut self.last_throttle_frame_number,
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
                self.app.render();
                self.app.frame_count += 1;
            }
            _ => {}
        }
    }
}

fn game_loop_iteration(
    app: &mut App,
    previous: &mut Instant,
    lag: &mut f32,
    fixed_time_step: f32,
    throttle_threshold: f32,
    throttle_frame_interval: usize,
    frame_counter: usize,
    last_throttle_frame_number: &mut usize,
) {
    let elapsed = previous.elapsed();
    *previous = Instant::now();
    *lag += elapsed.as_secs_f32();

    // Update rate stabilization loop.
    while *lag >= fixed_time_step {
        let time_step;
        if *lag >= throttle_threshold
            && (frame_counter - *last_throttle_frame_number >= throttle_frame_interval)
        {
            // Modify the delta time to let the game internals to fast-forward the
            // logic by the current lag.
            time_step = *lag;
            // Reset the lag to exit early from the loop, thus preventing its
            // potential infinite increase, that in its turn could hang up the game.
            *lag = 0.0;

            *last_throttle_frame_number = frame_counter;
        } else {
            time_step = fixed_time_step;
        }

        app.update(time_step, lag);

        // Additional check is needed, because the `update` call above could modify
        // the lag.
        if *lag >= fixed_time_step {
            *lag -= fixed_time_step;
        } else if *lag < 0.0 {
            // Prevent from going back in time.
            *lag = 0.0;
        }
    }

    for system_window in app.system_window_manager.get_ref().iter() {
        if let Some(window) = system_window.get_system_window::<WinitWindow>() {
            window.request_redraw();
        }
    }
}

impl<M: Message> WinitAppRunnerState<M> {
    fn new(app: App) -> Self {
        Self {
            app,
            app_exit: None,
            desired_update_rate: DEFAULT_UPDATE_RATE,
            throttle_threshold: 2.0 * DEFAULT_TIME_STEP,
            throttle_frame_interval: 5,
            lag: 0.0,
            previous: Instant::now(),
            last_throttle_frame_number: 0,
            _marker: PhantomData,
        }
    }
}

pub fn winit_runner<M: Message>(app: App, event_loop: EventLoop<M>) -> AppExit {
    let mut runner_state = WinitAppRunnerState::new(app);

    if let Err(err) = event_loop.run_app(&mut runner_state) {
        error!("Winit event loop returned an error: {err}");
    }
    // If everything is working correctly then the event loop only exits after it's sent an exit code.
    runner_state.app_exit.unwrap_or_else(|| {
        error!("Failed to receive an app exit code! This is a bug");
        AppExit::error()
    })
}
