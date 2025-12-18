use std::{
    num::NonZero,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
};

use draft_material::MaterialEffectLoader;
use draft_render::{
    GraphicsContext, InitializedGraphicsContext, RenderPipelineExt, WorldRenderer,
    initialize_render_server,
};
use draft_render_2d::create_core_2d_render_pipiline;
use draft_scene::SceneContainer;
use draft_window::{
    Error as WindoError, RawHandleWrapper, SystemWindowManager, Window, WindowWrapper,
};
use fyrox_core::task::TaskPool;
use fyrox_resource::{event::ResourceEvent, io::FsResourceIo, manager::ResourceManager};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    WindoError(#[from] WindoError),
}

pub trait IEventLoop {
    fn create_window(&self, window: &Window) -> WindowWrapper;
}

#[derive(Default)]
pub enum AppExit {
    #[default]
    Success,
    Error(NonZero<u8>),
}

impl AppExit {
    pub const fn error() -> Self {
        Self::Error(NonZero::<u8>::MIN)
    }
}

pub struct App {
    pub frame_count: usize,
    pub graphics_context: GraphicsContext,
    pub system_window_manager: SystemWindowManager,
    pub resource_manager: ResourceManager,
    pub scene_container: SceneContainer,
    _model_events_receiver: Receiver<ResourceEvent>,
}

impl App {
    pub fn new() -> App {
        let task_pool = Arc::new(TaskPool::new());
        let io = Arc::new(FsResourceIo);

        let resource_manager = ResourceManager::new(io, task_pool.clone());

        initialize_resource_manager_loaders(&resource_manager);

        let (rx, tx) = channel();

        resource_manager.state().event_broadcaster.add(rx);

        Self {
            graphics_context: Default::default(),
            system_window_manager: Default::default(),
            scene_container: Default::default(),
            frame_count: 0,
            resource_manager,
            _model_events_receiver: tx,
        }
    }

    pub fn destroy_graphics_context(&mut self) -> Result<(), AppError> {
        let graphics_context = match &self.graphics_context {
            GraphicsContext::Initialized(params) => params,
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already destroyed!".to_string(),
                ));
            }
        };
        let params = graphics_context.params.clone();
        self.graphics_context = GraphicsContext::Uninitialized(params);

        self.system_window_manager = SystemWindowManager::default();

        Ok(())
    }

    pub fn initialize_graphics_context<T: IEventLoop>(
        &mut self,
        event_loop: &T,
    ) -> Result<(), AppError> {
        let params = match &self.graphics_context {
            GraphicsContext::Uninitialized(params) => params.clone(),
            _ => {
                return Err(AppError::Custom(
                    "Graphics context is already initialized!".to_string(),
                )
                .into());
            }
        };

        let handle = self
            .system_window_manager
            .spawn_primary(params.window.clone());

        let winit_window = event_loop.create_window(&params.window);

        let wrapper = RawHandleWrapper::new(&winit_window).unwrap();
        let render_server = initialize_render_server(wrapper);

        self.system_window_manager.initialize_system_window(
            &render_server.instance,
            &render_server.device,
            &render_server.adapter,
            handle,
            winit_window,
        )?;

        let mut renderer = WorldRenderer::new(
            render_server,
            self.system_window_manager.clone(),
            &self.resource_manager,
        );

        renderer.insert_pipeline("core_2d", create_core_2d_render_pipiline());

        self.graphics_context =
            GraphicsContext::Initialized(InitializedGraphicsContext::new(renderer, params));

        Ok(())
    }

    pub fn update(&mut self, _dt: f32, _lag: &mut f32) {
        self.graphics_context.update();
    }

    pub fn render(&mut self) {
        self.graphics_context.render(&self.scene_container);
    }
}

pub(crate) fn initialize_resource_manager_loaders(resource_manager: &ResourceManager) {
    resource_manager.add_loader(MaterialEffectLoader);
}
