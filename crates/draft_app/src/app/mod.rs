use std::{
    mem::swap,
    num::NonZero,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
};

use draft_material::{IMaterial, MaterialEffectLoader};
use draft_render::{
    GraphicsContext, InitializedGraphicsContext, RenderPipelineExt, WorldRenderer,
    initialize_render_server,
};
use draft_render_2d::{Material2d, create_core_2d_render_pipiline};
use draft_scene::SceneContainer;
use draft_window::{
    Error as WindowError, RawHandleWrapper, SystemWindowManager, Window, WindowWrapper,
};
use fyrox_core::task::TaskPool;
use fyrox_resource::{event::ResourceEvent, io::FsResourceIo, manager::ResourceManager};
use thiserror::Error;

use crate::{PlaceholderPlugin, Plugin, PluginContainer, PluginsState};

type RunnerFn = Box<dyn FnOnce(App) -> ()>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    WindowError(#[from] WindowError),
    #[error("duplicate plugin {plugin_name:?}")]
    DuplicatePlugin { plugin_name: String },
}

fn run_once(mut _app: App) {}

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
    pub scene_container: SceneContainer,
    pub plugin_container: PluginContainer,
    pub runner: RunnerFn,
    _model_events_receiver: Receiver<ResourceEvent>,
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
            scene_container: Default::default(),
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

    pub fn run(&mut self) {
        if self.plugin_container.is_building_plugins() {
            panic!("App::run() was called while a plugin was building.");
        }

        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = core::mem::replace(self, App::empty());
        (runner)(app)
    }

    pub fn set_runner(&mut self, f: impl FnOnce(App) -> () + 'static) -> &mut Self {
        self.runner = Box::new(f);
        self
    }

    pub fn destroy_graphics_context(&mut self) -> Result<(), AppError> {
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

    pub fn finish(&mut self) {
        // plugins installed to main should see all sub-apps
        // do hokey pokey with a boxed zst plugin (doesn't allocate)
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

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        if matches!(self.plugins_state(), PluginsState::Finished) {
            panic!("Plugins cannot be added after App::finish() has been called.");
        }

        if let Err(AppError::DuplicatePlugin { plugin_name }) =
            self.add_boxed_plugin(Box::new(plugin))
        {
            panic!("Error adding plugin {plugin_name}: : plugin was already added in application")
        }

        self
    }

    pub fn initialize_graphics_context<T: IEventLoop>(
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

        let mut renderer = WorldRenderer::new(
            render_server,
            self.system_window_manager.clone(),
            &self.resource_manager,
        );

        renderer.insert_pipeline("core_2d", create_core_2d_render_pipiline());

        self.graphics_context =
            GraphicsContext::Initialized(InitializedGraphicsContext::new(renderer, params));

        for resource in Material2d::built_in_shaders() {
            self.resource_manager
                .register_built_in_resource(resource.clone());

            self.graphics_context.set_shader(resource.resource());
        }

        for resource in Material2d::built_in_material_effects() {
            self.resource_manager
                .register_built_in_resource(resource.clone());

            self.graphics_context
                .set_material_effect(resource.resource());
        }

        Ok(())
    }

    pub fn update(&mut self, dt: f32, _lag: &mut f32) {
        self.graphics_context.update(dt);
    }

    pub fn render(&mut self) {
        self.graphics_context.render(&self.scene_container);
    }
}

pub(crate) fn initialize_resource_manager_loaders(resource_manager: &ResourceManager) {
    resource_manager.add_loader(MaterialEffectLoader);
}
