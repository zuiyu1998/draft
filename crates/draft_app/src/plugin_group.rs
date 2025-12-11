use std::{
    any::TypeId,
    collections::{HashMap, hash_map::Entry},
};

use log::debug;

use crate::{App, AppError, Plugin};

struct PluginEntry {
    plugin: Box<dyn Plugin>,
    enabled: bool,
}

pub struct PluginGroupBuilder {
    group_name: String,
    plugins: HashMap<TypeId, PluginEntry>,
    order: Vec<TypeId>,
}

impl PluginGroupBuilder {
    pub fn finish(mut self, app: &mut App) {
        for ty in &self.order {
            if let Some(entry) = self.plugins.remove(ty)
                && entry.enabled
            {
                debug!("added plugin: {}", entry.plugin.name());
                if let Err(AppError::DuplicatePlugin { plugin_name }) =
                    app.add_boxed_plugin(entry.plugin)
                {
                    panic!(
                        "Error adding plugin {} in group {}: plugin was already added in application",
                        plugin_name, self.group_name
                    );
                }
            }
        }
    }

    pub fn try_set<T: Plugin>(mut self, plugin: T) -> Result<Self, (Self, T)> {
        match self.plugins.entry(TypeId::of::<T>()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().plugin = Box::new(plugin);

                Ok(self)
            }
            Entry::Vacant(_) => Err((self, plugin)),
        }
    }

    pub fn set<T: Plugin>(self, plugin: T) -> Self {
        self.try_set(plugin).unwrap_or_else(|_| {
            panic!(
                "{} does not exist in this PluginGroup",
                core::any::type_name::<T>(),
            )
        })
    }
}

pub trait PluginGroup: Sized {
    fn build(self) -> PluginGroupBuilder;

    fn name() -> String {
        core::any::type_name::<Self>().to_string()
    }

    fn set<T: Plugin>(self, plugin: T) -> PluginGroupBuilder {
        self.build().set(plugin)
    }
}
