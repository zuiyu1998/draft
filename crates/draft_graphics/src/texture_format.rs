use fyrox_core::{reflect::*, visitor::*};

// use wgpu_types::TextureFormat;

use crate::TextureAspect;

#[derive(Debug, Clone, Visit, Reflect, Default, Copy)]
pub enum TextureFormat {
    R8Unorm,
    #[default]
    Rgba8UnormSrgb,
    Rgba8Unorm,
    R16Uint,
    Rg16Uint,
    Rgba16Unorm,
    Rgba32Float,
}

impl TextureFormat {
    pub fn block_dimensions(&self) -> (u32, u32) {
        match self {
            Self::R8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Unorm
            | Self::R16Uint
            | Self::Rg16Uint
            | Self::Rgba16Unorm
            | Self::Rgba32Float => (1, 1),
        }
    }

    pub fn block_copy_size(&self, _aspect: Option<TextureAspect>) -> Option<u32> {
        match *self {
            Self::R8Unorm => Some(1),
            Self::R16Uint => Some(2),
            Self::Rgba8UnormSrgb | Self::Rgba8Unorm | Self::Rg16Uint => Some(4),
            Self::Rgba16Unorm => Some(8),
            Self::Rgba32Float => Some(16),
        }
    }
}
