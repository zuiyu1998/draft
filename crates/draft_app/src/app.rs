use std::mem::take;

use crate::{GraphicsContext, Plugin, PluginContainer};

type RunnerFn = Box<dyn FnOnce(App)>;

pub struct AppInitializeParams {}

pub struct App {
    graphics_context: GraphicsContext,
    plugin_container: PluginContainer,

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
            plugin_container: Default::default(),
        }
    }

    pub fn initialize(&mut self, _params: AppInitializeParams) {}

    pub fn destroy(&mut self) {}

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.add_boxed_plugin(Box::new(plugin))
    }

    fn add_boxed_plugin(&mut self, plugin: Box<dyn Plugin>) -> &mut Self {
        plugin.build(self);

        self.plugin_container.add_boxed_plugin(plugin);

        self
    }

    pub fn finished(&mut self) {
        let plugin_container = take(&mut self.plugin_container);
        plugin_container.finished(self);
        self.plugin_container = plugin_container;
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
