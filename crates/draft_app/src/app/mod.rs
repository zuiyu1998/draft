use std::num::NonZero;

use draft_render::GraphicsContext;

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
}

impl App {
    pub fn new() -> App {
        App::empty()
    }

    pub fn empty() -> App {
        Self {
            graphics_context: Default::default(),
        }
    }
}
