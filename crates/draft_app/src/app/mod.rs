use std::{
    mem::swap,
    num::NonZero,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
};

use draft_material::MaterialEffectLoader;
use draft_render::{
    GraphicsContext, InitializedGraphicsContext, RenderWorld, World, WorldRenderer,
    initialize_render_server,
};
use draft_window::{
    Error as WindowError, RawHandleWrapper, SystemWindowManager, Window, WindowWrapper,
};
use fyrox_core::task::TaskPool;
use fyrox_resource::{event::ResourceEvent, io::FsResourceIo, manager::ResourceManager};
use thiserror::Error;

use crate::{PlaceholderPlugin, Plugin, PluginContainer, Plugins, PluginsState};

type RunnerFn = Box<dyn FnOnce(App) -> AppExit>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    WindowError(#[from] WindowError),
    #[error("duplicate plugin {plugin_name:?}")]
    DuplicatePlugin { plugin_name: String },
}

fn run_once(mut _app: App) -> AppExit {
    AppExit::Success
}

pub trait IEventLoop {
    fn create_window(&self, window: &Window) -> WindowWrapper;
}

#[derive(Default)]
pub enum AppExit {
    #[default]
    Success,
    Error(NonZero<u8>),
}

impl AppExit {
    pub const fn error() -> Self {
        Self::Error(NonZero::<u8>::MIN)
    }
}

pub(crate) struct HokeyPokey;
impl Plugin for HokeyPokey {
    fn build(&self, _: &mut App) {}
}

pub struct App {
    pub frame_count: usize,
    pub graphics_context: GraphicsContext,
    pub system_window_manager: SystemWindowManager,
    pub resource_manager: ResourceManager,
    pub world_container: WorldContainer,
    pub plugin_container: PluginContainer,
    pub runner: RunnerFn,
    _model_events_receiver: Receiver<ResourceEvent>,
}

struct EmptyWorld;

impl World for EmptyWorld {
    fn prepare(&self, _render_world: &mut RenderWorld) {}
}

pub struct WorldContainer(Box<dyn World>);

impl WorldContainer {
    pub fn set_world<W>(&mut self, world: W)
    where
        W: World,
    {
        self.0 = Box::new(world);
    }
}

impl Default for WorldContainer {
    fn default() -> Self {
        Self(Box::new(EmptyWorld))
    }
}

impl App {
    pub fn new() -> App {
        App::empty()
    }

    pub fn empty() -> App {
        let task_pool = Arc::new(TaskPool::new());
        let io = Arc::new(FsResourceIo);

        let resource_manager = ResourceManager::new(io, task_pool.clone());

        initialize_resource_manager_loaders(&resource_manager);

        let (rx, tx) = channel();

        resource_manager.state().event_broadcaster.add(rx);

        Self {
            graphics_context: Default::default(),
            system_window_manager: Default::default(),
            world_container: Default::default(),
            plugin_container: Default::default(),
            frame_count: 0,
            resource_manager,
            _model_events_receiver: tx,
            runner: Box::new(run_once),
        }
    }

    pub(crate) fn add_boxed_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
    ) -> Result<&mut Self, AppError> {
        if plugin.is_unique() && self.plugin_container.plugin_names.contains(plugin.name()) {
            Err(AppError::DuplicatePlugin {
                plugin_name: plugin.name().to_string(),
            })?;
        }

        // Reserve position in the plugin registry. If the plugin adds more plugins,
        // they'll all end up in insertion order.
        let index = self.plugin_container.plugin_registry.len();
        self.plugin_container
            .plugin_registry
            .push(Box::new(PlaceholderPlugin));

        self.plugin_container.plugin_build_depth += 1;

        plugin.build(self);

        self.plugin_container
            .plugin_names
            .insert(plugin.name().to_string());
        self.plugin_container.plugin_build_depth -= 1;

        self.plugin_container.plugin_registry[index] = plugin;

        Ok(self)
    }

    pub fn run(&mut self) -> AppExit {
        if self.plugin_container.is_building_plugins() {
            panic!("App::run() was called while a plugin was building.");
        }

        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) -> AppExit + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    fn destroy_graphics_context(&mut self) -> Result<(), AppError> {
        let graphics_context = match &self.graphics_context {
            GraphicsContext::Initialized(params) => params,
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already destroyed!".to_string(),
                ));
            }
        };
        let params = graphics_context.params.clone();
        self.graphics_context = GraphicsContext::Uninitialized(params);

        self.system_window_manager = SystemWindowManager::default();

        Ok(())
    }

    pub fn plugins_state(&mut self) -> PluginsState {
        self.plugin_container.plugins_state
    }

    pub fn finish<T: IEventLoop>(&mut self, event_loop: &T) {
        if let Err(e) = self.initialize_graphics_context(event_loop) {
            panic!("Initialize graphics context failed. The error is {}.", e);
        }

        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);
        for i in 0..self.plugin_container.plugin_registry.len() {
            swap(
                &mut self.plugin_container.plugin_registry[i],
                &mut hokeypokey,
            );

            hokeypokey.finish(self);
            swap(
                &mut self.plugin_container.plugin_registry[i],
                &mut hokeypokey,
            );
        }
        self.plugin_container.plugins_state = PluginsState::Finished;
    }

    pub fn cleanup(&mut self) {
        if let Err(e) = self.destroy_graphics_context() {
            panic!("Destroy graphics context failed. The error is {}.", e);
        }

        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);
        for i in 0..self.plugin_container.plugin_registry.len() {
            core::mem::swap(
                &mut self.plugin_container.plugin_registry[i],
                &mut hokeypokey,
            );

            hokeypokey.cleanup(self);
            core::mem::swap(
                &mut self.plugin_container.plugin_registry[i],
                &mut hokeypokey,
            );
        }
        self.plugin_container.plugins_state = PluginsState::Cleaned;
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        if matches!(
            self.plugins_state(),
            PluginsState::Finished | PluginsState::Cleaned
        ) {
            panic!(
                "Plugins cannot be added after App::finish() or App::cleanup() has been called."
            );
        }

        plugins.add_to_app(self);

        self
    }

    fn initialize_graphics_context<T: IEventLoop>(
        &mut self,
        event_loop: &T,
    ) -> Result<(), AppError> {
        let params = match &self.graphics_context {
            GraphicsContext::Uninitialized(params) => params.clone(),
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already initialized!".to_string(),
                )
                .into());
            }
        };

        let handle = self
            .system_window_manager
            .spawn_primary(params.window.clone());

        let winit_window = event_loop.create_window(&params.window);

        let wrapper = RawHandleWrapper::new(&winit_window).unwrap();
        let render_server = initialize_render_server(wrapper);

        self.system_window_manager.initialize_system_window(
            &render_server.instance,
            &render_server.device,
            &render_server.adapter,
            handle,
            winit_window,
        )?;

        let renderer = WorldRenderer::new(
            render_server,
            self.system_window_manager.clone(),
            &self.resource_manager,
        );

        self.graphics_context =
            GraphicsContext::Initialized(InitializedGraphicsContext::new(renderer, params));

        Ok(())
    }

    pub fn update(&mut self, dt: f32, _lag: &mut f32) {
        self.graphics_context.update(dt);
    }

    pub fn render(&mut self) {
        self.graphics_context.render(&self.world_container.0);
    }
}

pub(crate) fn initialize_resource_manager_loaders(resource_manager: &ResourceManager) {
    resource_manager.add_loader(MaterialEffectLoader);
}
