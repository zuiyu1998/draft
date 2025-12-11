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
    pub fn start<PG: PluginGroup>() -> Self {
        Self {
            group_name: PG::name(),
            plugins: Default::default(),
            order: Default::default(),
        }
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
