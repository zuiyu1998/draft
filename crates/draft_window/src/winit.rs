use winit::window::Window;

use crate::{ISystemWindow, PhysicalSize};

impl ISystemWindow for Window {
    fn inner_size(&self) -> PhysicalSize {
        let size = self.inner_size();
        PhysicalSize {
            width: size.width,
            height: size.height,
        }
    }
}
