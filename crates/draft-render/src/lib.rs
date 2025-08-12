mod error;
mod phase;
mod utils;
mod world;

pub mod frame_graph;
pub mod render_resource;

pub use draft_gfx_base as gfx_base;

pub use error::*;
pub use phase::*;
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
