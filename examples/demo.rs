use draft_app::app::{App, Executor, WinitExecutor};

fn main() {
    let app = App::empty();

    let mut executor = WinitExecutor::default();
    executor.run(app);
}
