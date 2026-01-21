mod group;

pub use group::*;

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
    /// Cleanup has been executed for all plugins added.
    Cleaned,
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

    fn cleanup(&self, _app: &mut App) {}

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

pub trait Plugins<Marker>: sealed::Plugins<Marker> {}

impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}

mod sealed {
    use crate::{App, AppError, Plugin, PluginGroup};

    pub trait Plugins<Marker> {
        fn add_to_app(self, app: &mut App);
    }

    pub struct PluginMarker;
    pub struct PluginGroupMarker;

    impl<P: Plugin> Plugins<PluginMarker> for P {
        #[track_caller]
        fn add_to_app(self, app: &mut App) {
            if let Err(AppError::DuplicatePlugin { plugin_name }) =
                app.add_boxed_plugin(Box::new(self))
            {
                panic!(
                    "Error adding plugin {plugin_name}: : plugin was already added in application"
                )
            }
        }
    }

    impl<P: PluginGroup> Plugins<PluginGroupMarker> for P {
        #[track_caller]
        fn add_to_app(self, app: &mut App) {
            self.build().finish(app);
        }
    }
}
