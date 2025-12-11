mod state;

use draft_app::{App, Plugin};
use state::winit_runner;
use std::marker::PhantomData;
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

#[derive(Default)]
pub struct WinitPlugin<M: Message = WakeUp> {
    pub run_on_any_thread: bool,
    marker: PhantomData<M>,
}

#[derive(Default)]
pub struct WakeUp;

impl Message for WakeUp {}

pub trait Message: Send + Sync + 'static {}

impl<T: Message> Plugin for WinitPlugin<T> {
    fn build(&self, app: &mut App) {
        let mut event_loop_builder = EventLoop::<T>::with_user_event();

        event_loop_builder.with_any_thread(self.run_on_any_thread);

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        app.set_runner(|app| winit_runner(app, event_loop));
    }
}
