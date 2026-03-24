#[cfg(feature = "winit")]
mod winit;

use crate::app::App;

#[cfg(feature = "winit")]
pub use winit::*;

pub trait Executor: 'static {
    fn run(&mut self, app: App);
}
