use core::any::Any;
use downcast_rs::Downcast;

use crate::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginsState {
    Adding,
    Ready,
    Finished,
    Cleaned,
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

pub(crate) struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, _app: &mut App) {}
}

pub(crate) struct HokeyPokey;

impl Plugin for HokeyPokey {
    fn build(&self, _: &mut App) {}
}

pub trait Plugins<Marker>: sealed::Plugins<Marker> {}

impl<Marker, T> Plugins<Marker> for T where T: sealed::Plugins<Marker> {}

mod sealed {
    use std::boxed::Box;
    use variadics_please::all_tuples;

    use crate::{App, AppError, Plugin, PluginGroup};

    pub trait Plugins<Marker> {
        fn add_to_app(self, app: &mut App);
    }

    pub struct PluginMarker;
    pub struct PluginGroupMarker;
    pub struct PluginsTupleMarker;

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

    macro_rules! impl_plugins_tuples {
        ($(#[$meta:meta])* $(($param: ident, $plugins: ident)),*) => {
            $(#[$meta])*
            impl<$($param, $plugins),*> Plugins<(PluginsTupleMarker, $($param,)*)> for ($($plugins,)*)
            where
                $($plugins: Plugins<$param>),*
            {
                #[expect(
                    clippy::allow_attributes,
                    reason = "This is inside a macro, and as such, may not trigger in all cases."
                )]
                #[allow(non_snake_case, reason = "`all_tuples!()` generates non-snake-case variable names.")]
                #[allow(unused_variables, reason = "`app` is unused when implemented for the unit type `()`.")]
                #[track_caller]
                fn add_to_app(self, app: &mut App) {
                    let ($($plugins,)*) = self;
                    $($plugins.add_to_app(app);)*
                }
            }
        }
    }

    all_tuples!(impl_plugins_tuples, 0, 15, P, S);
}
