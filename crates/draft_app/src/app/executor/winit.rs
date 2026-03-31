use std::time::{Duration, Instant};

use crate::app::{App, ApplicationLoopController, game_loop_iteration};

use super::Executor;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

impl ApplicationLoopController for ActiveEventLoop {}

pub enum WinitUserEvent {
    /// Dummy event that just wakes up the winit event loop
    WakeUp,
    /// Tell winit that a window needs to be created
    WindowAdded,
}

pub struct WinitExecutor {
    pub run_on_any_thread: bool,

    desired_update_rate: f32,
    throttle_threshold: f32,
    throttle_frame_interval: usize,
}

impl WinitExecutor {
    /// Default update rate in frames per second.
    pub const DEFAULT_UPDATE_RATE: f32 = 60.0;
    /// Default time step (in seconds).
    pub const DEFAULT_TIME_STEP: f32 = 1.0 / Self::DEFAULT_UPDATE_RATE;

    pub fn new() -> Self {
        Self {
            run_on_any_thread: true,
            desired_update_rate: Self::DEFAULT_UPDATE_RATE,
            throttle_threshold: 2.0 * Self::DEFAULT_TIME_STEP,
            throttle_frame_interval: 5,
        }
    }
}

impl Default for WinitExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor for WinitExecutor {
    fn run(&mut self, app: App) {
        let mut event_loop_builder = EventLoop::<WinitUserEvent>::with_user_event();
        use winit::platform::windows::EventLoopBuilderExtWindows;
        event_loop_builder.with_any_thread(self.run_on_any_thread);

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        let mut runner_state = WinitAppRunnerState::new(
            app,
            self.desired_update_rate,
            self.throttle_threshold,
            self.throttle_frame_interval,
        );
        if let Err(err) = event_loop.run_app(&mut runner_state) {
            println!("winit event loop returned an error: {err}");
        }
    }
}

pub(crate) struct WinitAppRunnerState {
    app: App,
    previous: Instant,
    lag: f32,
    frame_counter: usize,
    fixed_time_step: f32,
    throttle_threshold: f32,
    throttle_frame_interval: usize,
    last_throttle_frame_number: usize,
}

impl WinitAppRunnerState {
    fn new(
        app: App,
        desired_update_rate: f32,
        throttle_threshold: f32,
        throttle_frame_interval: usize,
    ) -> Self {
        let fixed_time_step = 1.0 / desired_update_rate;
        let lag = fixed_time_step;

        Self {
            app,
            previous: Instant::now(),
            lag,
            fixed_time_step,
            throttle_threshold,
            throttle_frame_interval,
            frame_counter: 0,
            last_throttle_frame_number: 0,
        }
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        game_loop_iteration(
            &mut self.app,
            event_loop,
            &mut self.previous,
            &mut self.lag,
            self.fixed_time_step,
            self.throttle_threshold,
            self.throttle_frame_interval,
            self.frame_counter,
            &mut self.last_throttle_frame_number,
        );
        self.frame_counter += 1;

        // Only sleep for two-third of the remaining time step because thread::sleep tends to overshoot.
        let sleep_time =
            (self.fixed_time_step - self.previous.elapsed().as_secs_f32()).max(0.0) * 0.66666;

        if sleep_time > 0.0 {
            std::thread::sleep(Duration::from_secs_f32(sleep_time));
        }
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
