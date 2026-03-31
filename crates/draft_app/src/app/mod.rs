mod executor;
mod plugin;

use std::sync::Arc;

pub use executor::*;
use fyrox_resource::{io::FsResourceIo, manager::ResourceManager};
pub use plugin::*;

use draft_render::{FrameworkError, renderer::WorldRenderer};
use draft_window::SystemWindowManager;

use crate::{
    graphics_context::{GraphicsContext, InitializedGraphicsContext, RenderServerConstructor},
    scene::Scene,
};

pub struct App {
    pub scene: Scene,
    pub system_window_manager: SystemWindowManager,
    pub graphics_context: GraphicsContext,
    pub(crate) plugin_container: PluginContainer,
    resource_manager: ResourceManager,
}

impl App {
    pub fn empty() -> Self {
        let resource_manager =
            ResourceManager::new(Arc::new(FsResourceIo), Arc::new(Default::default()));

        Self {
            scene: Scene::empty(),
            graphics_context: GraphicsContext::default(),
            plugin_container: PluginContainer::default(),
            system_window_manager: Default::default(),
            resource_manager,
        }
    }

    pub fn update<C: ApplicationLoopController>(
        &mut self,
        _dt: f32,
        _controller: &C,
        _lag: &mut f32,
    ) {
    }

    pub fn render(&mut self) {
        self.graphics_context.render(&mut self.scene);
    }

    pub fn initialize_graphics_context<T: RenderServerConstructor>(
        &mut self,
        constructor: &T,
    ) -> Result<(), FrameworkError> {
        if let GraphicsContext::Uninitialized(params) = &self.graphics_context {
            let (render_server, window) =
                constructor.construct(&params.render_server_setting, params.window.clone())?;

            self.system_window_manager
                .state()
                .spawn_primary_window(window);

            let renderer = WorldRenderer::new(
                render_server,
                self.system_window_manager.clone(),
                &self.resource_manager,
            );

            self.graphics_context = GraphicsContext::Initialized(InitializedGraphicsContext {
                params: params.clone(),
                renderer,
            });

            Ok(())
        } else {
            panic!("Graphics context is already initialized!");
        }
    }

    pub fn destroy_graphics_context(&mut self) {}

    pub fn finish(&mut self) {
        let mut hokeypokey: Box<dyn Plugin> = Box::new(HokeyPokey);
        for i in 0..self.plugin_container.plugins.len() {
            core::mem::swap(&mut self.plugin_container.plugins[i], &mut hokeypokey);

            hokeypokey.finish(self);
            core::mem::swap(&mut self.plugin_container.plugins[i], &mut hokeypokey);
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.add_boxed_plugin(Box::new(plugin));

        self
    }

    fn add_boxed_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.build(self);
        self.plugin_container.plugins.push(plugin);
    }
}
