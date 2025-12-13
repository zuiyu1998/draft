use std::{
    num::NonZero,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
};

use draft_render::{EmptyWorld, GraphicsContext, MaterialEffectLoader};
use draft_window::Windows;
use fyrox_core::task::TaskPool;
use fyrox_resource::{event::ResourceEvent, io::FsResourceIo, manager::ResourceManager};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("custom error: {0}")]
    Custom(String),
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
    pub windows: Windows,
    pub resource_manager: ResourceManager,

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
            windows: Default::default(),
            frame_count: 0,
            resource_manager,
            _model_events_receiver: tx,
        }
    }

    pub fn update(&mut self, _dt: f32, _lag: &mut f32) {}

    pub fn render(&mut self) {
        if let GraphicsContext::Initialized(graphics_context) = &mut self.graphics_context {
            graphics_context.renderer.render(&EmptyWorld::default());
        }
    }
}

pub(crate) fn initialize_resource_manager_loaders(resource_manager: &ResourceManager) {
    resource_manager.add_loader(MaterialEffectLoader);
}
