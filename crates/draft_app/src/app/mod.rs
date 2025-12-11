use std::num::NonZero;

#[derive(Default)]
pub enum AppExit {
    #[default]
    Success,

    Error(NonZero<u8>),
}

type RunnerFn = Box<dyn FnOnce(App) -> AppExit>;

pub struct App {
    pub(crate) runner: RunnerFn,
}

impl App {
    pub fn empty() -> App {
        Self {
            runner: Box::new(run_once),
        }
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) -> AppExit + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    pub fn run(&mut self) -> AppExit {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }
}

fn run_once(mut _app: App) -> AppExit {
    AppExit::Success
}
