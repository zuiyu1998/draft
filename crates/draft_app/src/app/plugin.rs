use std::any::Any;

use downcast_rs::{Downcast, impl_downcast};

use crate::app::App;

#[derive(Default)]
pub struct PluginContainer {
    pub plugins: Vec<Box<dyn Plugin>>,
}

pub(crate) struct HokeyPokey;

impl Plugin for HokeyPokey {
    fn build(&self, _: &mut App) {}
}

pub trait Plugin: Downcast + Any + Send + Sync {
    fn build(&self, app: &mut App);

    fn finish(&self, _app: &mut App) {
        // do nothing
    }
}

impl_downcast!(Plugin);
