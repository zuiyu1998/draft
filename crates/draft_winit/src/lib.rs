mod state;
mod winit_windows;

use draft_app::{App, Executor};
use state::winit_runner;
use std::marker::PhantomData;
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

pub trait Message: Send + Sync + 'static {}

#[derive(Default)]
pub struct WinitExecutor<M: Message = WakeUp> {
    pub run_on_any_thread: bool,
    marker: PhantomData<M>,
}

#[derive(Default)]
pub struct WakeUp;

impl Message for WakeUp {}

impl<T: Message> Executor for WinitExecutor<T> {
    fn run(&mut self, app: App) {
        let mut event_loop_builder = EventLoop::<T>::with_user_event();
        event_loop_builder.with_any_thread(self.run_on_any_thread);

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        winit_runner(app, event_loop);
    }
}
