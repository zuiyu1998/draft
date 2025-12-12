use std::num::NonZero;

use draft_render::{EmptyWorld, GraphicsContext};
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
    pub frame_count: usize,
    pub graphics_context: GraphicsContext,
    pub windows: Windows,
}

impl App {
    pub fn new() -> App {
        App::empty()
    }

    pub fn update(&mut self, _dt: f32, _lag: &mut f32) {}

    pub fn render(&mut self) {
        if let GraphicsContext::Initialized(graphics_context) = &mut self.graphics_context {
            graphics_context.renderer.render(&EmptyWorld);
        }
    }

    pub fn empty() -> App {
        Self {
            graphics_context: Default::default(),
            windows: Default::default(),
            frame_count: 0,
        }
    }
}
