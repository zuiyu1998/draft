mod plugin;

pub use plugin::*;

use draft_render::GraphicsContext;

use crate::scene::Scene;

type RunnerFn = Box<dyn FnOnce(App)>;

fn run_once(mut _app: App) {}

pub struct App {
    pub(crate) scene: Scene,
    pub(crate) graphics_context: GraphicsContext,
    pub(crate) plugin_container: PluginContainer,

    pub(crate) runner: RunnerFn,
}

impl App {
    pub fn empty() -> Self {
        Self {
            scene: Scene::empty(),
            graphics_context: GraphicsContext::default(),
            runner: Box::new(run_once),
            plugin_container: PluginContainer::default(),
        }
    }

    pub fn finish(&mut self) {
        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);
        for i in 0..self.plugin_container.plugins.len() {
            core::mem::swap(&mut self.plugin_container.plugins[i], &mut hokeypokey);

            hokeypokey.finish(self);
            core::mem::swap(&mut self.plugin_container.plugins[i], &mut hokeypokey);
        }
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.add_boxed_plugin(Box::new(plugin));

        self
    }

    fn add_boxed_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.build(self);
        self.plugin_container.plugins.push(plugin);
    }

    pub fn run(&mut self) {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }
}
