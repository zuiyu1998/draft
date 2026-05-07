use crate::GraphicsContext;

type RunnerFn = Box<dyn FnOnce(App)>;

pub struct App {
    graphics_context: GraphicsContext,

    pub(crate) runner: RunnerFn,
}

fn run_once(_app: App) {
    println!("run_once")
}

impl App {
    pub fn empty() -> Self {
        App {
            runner: Box::new(run_once),
            graphics_context: Default::default(),
        }
    }

    pub fn update(&mut self) {
        self.graphics_context.update();
    }

    pub fn run(&mut self) {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }
}
