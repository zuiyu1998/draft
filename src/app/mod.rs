use std::{num::NonZero, sync::Arc};

use fyrox_core::task::TaskPool;
use fyrox_resource::{io::FsResourceIo, manager::ResourceManager};

use crate::{renderer::GraphicsContext, scene::SceneContainer};

type Runner = Box<dyn FnOnce(App) -> AppExit>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AppExit {
    #[default]
    Success,
    Error(NonZero<u8>),
}

fn run_once(_app: App) -> AppExit {
    AppExit::Success
}

pub struct App {
    runner: Runner,
    scene_container: SceneContainer,
    graphics_context: GraphicsContext,
    resource_manager: ResourceManager,
}

impl Default for App {
    fn default() -> Self {
        Self::empty()
    }
}

impl App {
    pub fn empty() -> Self {
        let task_pool = Arc::new(TaskPool::new());
        let resource_manager = ResourceManager::new(Arc::new(FsResourceIo), task_pool);

        Self {
            runner: Box::new(run_once),
            scene_container: Default::default(),
            graphics_context: GraphicsContext::Uninitialized(Default::default()),
            resource_manager,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}
