mod error;
mod scene_render_data;
mod utils;
mod world;

pub mod frame_graph;
pub mod gfx_base;
pub mod render_resource;
pub mod renderer;

pub use error::*;

pub use scene_render_data::*;
pub use utils::*;
pub use world::*;

pub use wgpu;

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
