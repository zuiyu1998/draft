use std::{any::Any, collections::HashSet};

use downcast_rs::Downcast;

use crate::App;

pub(crate) struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Plugins state in the application
#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Default)]
pub enum PluginsState {
    /// Plugins are being added.
    #[default]
    Adding,
    /// Finish has been executed for all plugins added.
    Finished,
}

#[derive(Default)]
pub struct PluginContainer {
    pub(crate) plugin_registry: Vec<Box<dyn Plugin>>,
    /// The names of plugins that have been added to this app. (used to track duplicates and
    /// already-registered plugins)
    pub(crate) plugin_names: HashSet<String>,
    /// Panics if an update is attempted while plugins are building.
    pub(crate) plugin_build_depth: usize,
    pub(crate) plugins_state: PluginsState,
}

impl PluginContainer {
    pub(crate) fn is_building_plugins(&self) -> bool {
        self.plugin_build_depth > 0
    }
}

pub trait Plugin: Downcast + Any + Send + Sync {
    fn build(&self, app: &mut App);

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {}

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}
