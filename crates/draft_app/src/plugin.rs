use core::any::Any;
use downcast_rs::Downcast;

use crate::App;

pub enum PluginsState {
    Adding,
    Finished,
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

pub(crate) struct HokeyPokey;

impl Plugin for HokeyPokey {
    fn build(&self, _: &mut App) {}
}
