use std::num::NonZero;

use draft_render::GraphicsContext;
use draft_window::Windows;

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
    pub graphics_context: GraphicsContext,
    pub windows: Windows
}

impl App {
    pub fn new() -> App {
        App::empty()
    }

    pub fn empty() -> App {
        Self {
            graphics_context: Default::default(),
            windows: Default::default()
        }
    }
}
