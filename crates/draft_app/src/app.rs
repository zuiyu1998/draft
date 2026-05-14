use std::mem::take;

use draft_render::{IWorld, WorldRenderer};
use draft_window::{SystemWindow, SystemWindowManager};

use crate::{
    GraphicsContext, InitializedGraphicsContext, Plugin, PluginContainer, RenderServerConstructor,
    World,
};

type RunnerFn = Box<dyn FnOnce(App)>;

pub struct AppInitializeParams {
    pub window: SystemWindow,
    pub render_server_constructor: RenderServerConstructor,
}

pub struct App {
    graphics_context: GraphicsContext,
    plugin_container: PluginContainer,
    system_window_manager: SystemWindowManager,
    world: World,

    pub(crate) runner: RunnerFn,
}

fn run_once(_app: App) {
    println!("run_once")
}

impl App {
    pub fn new() -> Self {
        App {
            runner: Box::new(run_once),
            graphics_context: Default::default(),
            plugin_container: Default::default(),
            system_window_manager: Default::default(),
            world: World::empty(),
        }
    }

    pub fn set_world<W: IWorld>(&mut self, world: W) -> &mut Self {
        self.world = World::new(world);
        self
    }

    pub fn initialize(&mut self, params: AppInitializeParams) {
        if let GraphicsContext::Uninitialized(ref graphics_context_params) = self.graphics_context {
            self.system_window_manager
                .spawn_primary_window(params.window.clone());

            let render_server =
                (params.render_server_constructor)(graphics_context_params, params.window);

            let mut renderer =
                WorldRenderer::new(render_server, self.system_window_manager.clone());

            renderer.initialize();

            self.graphics_context = GraphicsContext::Initialized(InitializedGraphicsContext {
                params: graphics_context_params.clone(),
                renderer,
            })
        }
    }

    pub fn destroy(&mut self) {
        if let GraphicsContext::Initialized(initialized_graphics_context) = &self.graphics_context {
            let params = initialized_graphics_context.params.clone();

            self.graphics_context = GraphicsContext::Uninitialized(params);
        }
    }

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

    pub fn render(&mut self) {
        self.graphics_context.render(&self.world);
    }

    pub fn update(&mut self) {
        self.graphics_context.update();
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    pub fn run(&mut self) {
        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::new());
        (runner)(app)
    }
}
