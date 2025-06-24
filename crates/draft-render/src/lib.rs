mod common;
mod error;
mod geometry;
mod material;
mod scene_render_data;
mod shader;
mod texture;

pub mod frame_graph;
pub mod gfx_base;
pub mod resource;

pub use common::*;
pub use error::*;
pub use geometry::*;
pub use material::*;
pub use scene_render_data::*;
pub use shader::*;
pub use texture::*;

pub use wgpu;

use crate::resource::{RenderDevice, RenderQueue};

#[derive(Default)]
pub struct RenderStorage {
    pub material_storage: MaterialStorage,
    pub geometry_storage: GeometryStorage,
}

pub struct RenderServer {
    pub device: RenderDevice,
    pub queue: RenderQueue,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
