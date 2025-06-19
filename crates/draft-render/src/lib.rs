mod common;
mod error;
mod geometry;
mod material;
mod scene_render_data;
mod shader;

pub use common::*;
pub use error::*;
pub use geometry::*;
pub use material::*;
pub use scene_render_data::*;
pub use shader::*;

pub use frame_graph;
pub use frame_graph::wgpu;

use frame_graph::{RenderDevice, RenderQueue};

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
