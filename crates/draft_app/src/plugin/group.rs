use std::{any::TypeId, collections::hash_map::Entry};

use fxhash::FxHashMap;
use tracing::{debug, warn};

use crate::{App, AppError, Plugin};

struct PluginEntry {
    plugin: Box<dyn Plugin>,
    enabled: bool,
}

pub struct PluginGroupBuilder {
    group_name: String,
    plugins: FxHashMap<TypeId, PluginEntry>,
    order: Vec<TypeId>,
}

impl PluginGroupBuilder {
    /// Start a new builder for the [`PluginGroup`].
    pub fn start<PG: PluginGroup>() -> Self {
        Self {
            group_name: PG::name(),
            plugins: Default::default(),
            order: Default::default(),
        }
    }

    // Insert the new plugin entry as enabled, and removes its previous ordering if it was
    // already present
    fn upsert_plugin_entry_state(
        &mut self,
        key: TypeId,
        plugin: PluginEntry,
        added_at_index: usize,
    ) {
        if let Some(entry) = self.plugins.insert(key, plugin) {
            if entry.enabled {
                warn!(
                    "You are replacing plugin '{}' that was not disabled.",
                    entry.plugin.name()
                );
            }
            if let Some(to_remove) = self
                .order
                .iter()
                .enumerate()
                .find(|(i, ty)| *i != added_at_index && **ty == key)
                .map(|(i, _)| i)
            {
                self.order.remove(to_remove);
            }
        }
    }

    fn upsert_plugin_state<T: Plugin>(&mut self, plugin: T, added_at_index: usize) {
        self.upsert_plugin_entry_state(
            TypeId::of::<T>(),
            PluginEntry {
                plugin: Box::new(plugin),
                enabled: true,
            },
            added_at_index,
        );
    }

    #[expect(
        clippy::should_implement_trait,
        reason = "This does not emulate the `+` operator, but is more akin to pushing to a stack."
    )]
    pub fn add<T: Plugin>(mut self, plugin: T) -> Self {
        let target_index = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert_plugin_state(plugin, target_index);
        self
    }

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

    pub fn set<T: Plugin>(self, plugin: T) -> Self {
        self.try_set(plugin).unwrap_or_else(|_| {
            panic!(
                "{} does not exist in this PluginGroup",
                core::any::type_name::<T>(),
            )
        })
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
}

#[macro_export]
macro_rules! plugin_group {
    {
        $(#[$group_meta:meta])*
        $vis:vis struct $group:ident {
            $(
                $(#[cfg(feature = $plugin_feature:literal)])?
                $(#[custom($plugin_meta:meta)])*
                $($plugin_path:ident::)* : $plugin_name:ident
            ),*
            $(
                $(,)?$(
                    #[plugin_group]
                    $(#[cfg(feature = $plugin_group_feature:literal)])?
                    $(#[custom($plugin_group_meta:meta)])*
                    $($plugin_group_path:ident::)* : $plugin_group_name:ident
                ),+
            )?
            $(
                $(,)?$(
                    #[doc(hidden)]
                    $(#[cfg(feature = $hidden_plugin_feature:literal)])?
                    $(#[custom($hidden_plugin_meta:meta)])*
                    $($hidden_plugin_path:ident::)* : $hidden_plugin_name:ident
                ),+
            )?

            $(,)?
        }
        $($(#[doc = $post_doc:literal])+)?
    } => {
        $(#[$group_meta])*
        ///
        $(#[doc = concat!(
            " - [`", stringify!($plugin_name), "`](" $(, stringify!($plugin_path), "::")*, stringify!($plugin_name), ")"
            $(, " - with feature `", $plugin_feature, "`")?
        )])*
       $($(#[doc = concat!(
            " - [`", stringify!($plugin_group_name), "`](" $(, stringify!($plugin_group_path), "::")*, stringify!($plugin_group_name), ")"
            $(, " - with feature `", $plugin_group_feature, "`")?
        )])+)?
        $(
            ///
            $(#[doc = $post_doc])+
        )?
        $vis struct $group;

        impl $crate::PluginGroup for $group {
            fn build(self) -> $crate::PluginGroupBuilder {
                let mut group = $crate::PluginGroupBuilder::start::<Self>();

                $(
                    $(#[cfg(feature = $plugin_feature)])?
                    $(#[$plugin_meta])*
                    {
                        const _: () = {
                            const fn check_default<T: Default>() {}
                            check_default::<$($plugin_path::)*$plugin_name>();
                        };

                        group = group.add(<$($plugin_path::)*$plugin_name>::default());
                    }
                )*
                $($(
                    $(#[cfg(feature = $plugin_group_feature)])?
                    $(#[$plugin_group_meta])*
                    {
                        const _: () = {
                            const fn check_default<T: Default>() {}
                            check_default::<$($plugin_group_path::)*$plugin_group_name>();
                        };

                        group = group.add_group(<$($plugin_group_path::)*$plugin_group_name>::default());
                    }
                )+)?
                $($(
                    $(#[cfg(feature = $hidden_plugin_feature)])?
                    $(#[$hidden_plugin_meta])*
                    {
                        const _: () = {
                            const fn check_default<T: Default>() {}
                            check_default::<$($hidden_plugin_path::)*$hidden_plugin_name>();
                        };

                        group = group.add(<$($hidden_plugin_path::)*$hidden_plugin_name>::default());
                    }
                )+)?

                group
            }
        }
    };
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
