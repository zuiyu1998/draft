use draft_render::GraphicsContext;

use crate::scene::Scene;

type RunnerFn = Box<dyn FnOnce(App)>;

#[derive(Debug, Default)]
pub enum AppLifeCycle {
    #[default]
    Build,
}

fn run_once(mut _app: App) {}

pub struct App {
    pub(crate) scene: Scene,
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) life_cycle: AppLifeCycle,

    pub(crate) runner: RunnerFn,
}

impl App {
    pub fn empty() -> Self {
        Self {
            scene: Scene::empty(),
            graphics_context: GraphicsContext::default(),
            life_cycle: AppLifeCycle::default(),
            runner: Box::new(run_once),
        }
    }

    pub fn run(&mut self) {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }
}
