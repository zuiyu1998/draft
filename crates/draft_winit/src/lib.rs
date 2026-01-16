use std::marker::PhantomData;

use draft_app::Plugin;
use winit::event_loop::EventLoop;

use crate::state::winit_runner;

mod state;

pub trait Message: Send + Sync + 'static {}

#[derive(Default)]
pub struct WakeUp;

impl Message for WakeUp {}

pub struct WinitPlugin<M: Message = WakeUp> {
    pub run_on_any_thread: bool,
    marker: PhantomData<M>,
}

impl<T: Message> Plugin for WinitPlugin<T> {
    fn build(&self, app: &mut draft_app::App) {
        let mut event_loop_builder = EventLoop::<T>::with_user_event();

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            event_loop_builder.with_any_thread(self.run_on_any_thread);
        }

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to build event loop");

        app.set_runner(|app| winit_runner(app, event_loop));
    }
}
