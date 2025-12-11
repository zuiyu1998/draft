use std::{collections::HashSet, num::NonZero};

use crate::{Plugin, PluginsState};

#[derive(Default)]
pub enum AppExit {
    #[default]
    Success,

    Error(NonZero<u8>),
}

type RunnerFn = Box<dyn FnOnce(App) -> AppExit>;

pub struct App {
    pub(crate) plugin_registry: Vec<Box<dyn Plugin>>,
    pub(crate) plugin_names: HashSet<String>,
    pub(crate) plugin_build_depth: usize,
    pub(crate) plugins_state: PluginsState,

    pub(crate) runner: RunnerFn,
}

impl App {
    pub fn empty() -> App {
        Self {
            plugin_registry: Vec::default(),
            plugin_names: HashSet::default(),
            plugin_build_depth: 0,
            plugins_state: PluginsState::Adding,
            runner: Box::new(run_once),
        }
    }

    pub fn finish(&mut self) {
        let mut hokeypokey: Box<dyn Plugin> = Box::new(crate::HokeyPokey);

        for i in 0..self.plugin_registry.len() {
            core::mem::swap(&mut self.plugin_registry[i], &mut hokeypokey);
            hokeypokey.finish(self);

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
}

fn run_once(mut _app: App) -> AppExit {
    AppExit::Success
}
