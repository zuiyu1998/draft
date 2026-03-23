mod plugin;

use draft_render::{
    FrameworkError, WorldRenderer,
    render_server::{RenderServerConstructor, RenderServerSetting},
};
use draft_window::Window;
pub use plugin::*;

use crate::scene::Scene;

type RunnerFn = Box<dyn FnOnce(App)>;

fn run_once(mut _app: App) {}

pub struct App {
    pub scene: Scene,
    pub graphics_context: GraphicsContext,
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

    pub fn initialize_graphics_context<T: RenderServerConstructor>(
        &mut self,
        constructor: &T,
    ) -> Result<(), FrameworkError> {
        if let GraphicsContext::Uninitialized(params) = &self.graphics_context {
            let (render_server, window) =
                constructor.construct(&params.render_server_setting, params.window.clone())?;

            let renderer = WorldRenderer::new(render_server);

            self.graphics_context = GraphicsContext::Initialized(InitializedGraphicsContext {
                params: params.clone(),
                renderer,
            });

            Ok(())
        } else {
            panic!("Graphics context is already initialized!");
        }
    }

    pub fn destroy_graphics_context(&mut self) {}

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

pub enum GraphicsContext {
    Initialized(InitializedGraphicsContext),
    Uninitialized(GraphicsContextParams),
}

impl Default for GraphicsContext {
    fn default() -> Self {
        GraphicsContext::Uninitialized(GraphicsContextParams {
            render_server_setting: Default::default(),
            window: Default::default(),
        })
    }
}

pub struct InitializedGraphicsContext {
    pub params: GraphicsContextParams,

    pub renderer: WorldRenderer,
}

#[derive(Debug, Clone)]
pub struct GraphicsContextParams {
    pub render_server_setting: RenderServerSetting,
    pub window: Window,
}
