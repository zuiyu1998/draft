use log::debug;

use crate::{
    HokeyPokey, PlaceholderPlugin, Plugin, Plugins, PluginsState, renderer::GraphicsContext,
};
use std::{collections::HashSet, num::NonZero};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("duplicate plugin {plugin_name:?}")]
    DuplicatePlugin { plugin_name: String },
    #[error("custom error: {0}")]
    Custom(String),
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

type RunnerFn = Box<dyn FnOnce(App) -> AppExit>;

pub struct App {
    pub(crate) plugin_registry: Vec<Box<dyn Plugin>>,
    pub(crate) plugin_names: HashSet<String>,
    pub(crate) plugin_build_depth: usize,
    pub(crate) plugins_state: PluginsState,

    pub(crate) runner: RunnerFn,

    pub graphics_context: GraphicsContext,
}

impl App {
    pub fn new() -> App {
        App::empty()
    }

    pub fn empty() -> App {
        Self {
            plugin_registry: Vec::default(),
            plugin_names: HashSet::default(),
            plugin_build_depth: 0,
            plugins_state: PluginsState::Adding,
            runner: Box::new(run_once),
            graphics_context: GraphicsContext::default(),
        }
    }

    pub fn plugins_state(&mut self) -> PluginsState {
        let plugins_state = match self.plugins_state {
            PluginsState::Adding => {
                let mut state = PluginsState::Ready;
                let plugins = core::mem::take(&mut self.plugin_registry);
                for plugin in &plugins {
                    if !plugin.ready(self) {
                        state = PluginsState::Adding;
                        break;
                    }
                }
                self.plugin_registry = plugins;
                state
            }
            state => state,
        };

        plugins_state
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        if matches!(
            self.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }
        plugins.add_to_app(self);
        self
    }

    pub fn finish(&mut self) {
        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);

        for i in 0..self.plugin_registry.len() {
            core::mem::swap(&mut self.plugin_registry[i], &mut hokeypokey);
            hokeypokey.finish(self);

            core::mem::swap(&mut self.plugin_registry[i], &mut hokeypokey);
        }

        self.plugins_state = PluginsState::Finished;
    }

    pub fn cleanup(&mut self) {
        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);
        for i in 0..self.plugin_registry.len() {
            core::mem::swap(&mut self.plugin_registry[i], &mut hokeypokey);
            hokeypokey.cleanup(self);

            core::mem::swap(&mut self.plugin_registry[i], &mut hokeypokey);
        }

        self.plugins_state = PluginsState::Finished;
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

    pub(crate) fn add_boxed_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
    ) -> Result<&mut Self, AppError> {
        debug!("added plugin: {}", plugin.name());

        if plugin.is_unique() && self.plugin_names.contains(plugin.name()) {
            Err(AppError::DuplicatePlugin {
                plugin_name: plugin.name().to_string(),
            })?;
        }

        let index = self.plugin_registry.len();

        self.plugin_registry.push(Box::new(PlaceholderPlugin));

        self.plugin_build_depth += 1;

        plugin.build(self);

        self.plugin_names.insert(plugin.name().to_string());
        self.plugin_build_depth -= 1;

        self.plugin_registry[index] = plugin;

        Ok(self)
    }
}

fn run_once(mut _app: App) -> AppExit {
    AppExit::Success
}
