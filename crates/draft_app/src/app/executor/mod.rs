#[cfg(feature = "winit")]
mod winit;

use crate::app::App;

use fyrox_core::instant::Instant;
#[cfg(feature = "winit")]
pub use winit::*;

pub trait ApplicationLoopController {}

pub fn game_loop_iteration<C: ApplicationLoopController>(
    app: &mut App,
    controller: &C,
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

        app.update(time_step, controller, lag);

        // Additional check is needed, because the `update` call above could modify
        // the lag.
        if *lag >= fixed_time_step {
            *lag -= fixed_time_step;
        } else if *lag < 0.0 {
            // Prevent from going back in time.
            *lag = 0.0;
        }
    }

    app.graphics_context.request_redraw();
}

pub trait Executor: 'static {
    fn run(&mut self, app: App);
}
