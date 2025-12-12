pub mod prelude;

pub use draft_app as app;
pub use draft_render as render;

use draft_app::{App, Executor};
use draft_winit::{WakeUp, WinitExecutor};

pub fn run(app: App) {
    let mut executor = WinitExecutor::<WakeUp>::default();

    executor.run(app);
}
