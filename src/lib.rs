pub mod common;
pub mod material;
pub mod shader;

pub use common::*;
pub use material::*;
pub use shader::*;

pub use frame_graph;
pub use frame_graph::wgpu;

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
