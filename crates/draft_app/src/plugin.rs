use std::any::Any;

use downcast_rs::{Downcast, impl_downcast};

use crate::App;

pub trait Plugin: Any + Send + Sync + Downcast {
    fn build(&self, app: &mut App);

    fn finished(&self, _app: &mut App);
}

impl_downcast!(Plugin);

#[derive(Default)]
pub struct PluginContainer {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginContainer {
    pub fn add_boxed_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn finished(&self, app: &mut App) {
        for plugin in self.plugins.iter() {
            plugin.finished(app);
        }
    }
}
