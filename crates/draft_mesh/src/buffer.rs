use std::alloc::Layout;

use fyrox_core::{
    reflect::*,
    visitor::{pod::PodVecView, *},
};

#[derive(Reflect, Clone, Debug)]
pub struct BytesStorage {
    bytes: Vec<u8>,
    #[reflect(hidden)]
    layout: Layout,
}

impl Visit for BytesStorage {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut bytes_adapter = PodVecView::from_pod_vec(&mut self.bytes);
        if bytes_adapter.visit(name, visitor).is_err() {
            let mut bytes = Vec::<u8>::new();
            bytes.visit(name, visitor)?;
            self.bytes = bytes;
        }

        if visitor.is_reading() {
            self.layout = Layout::array::<u8>(self.bytes.capacity()).unwrap();
        }
        Ok(())
    }
}

impl Default for BytesStorage {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            layout: Layout::array::<u8>(0).unwrap(),
        }
    }
}

impl Drop for BytesStorage {
    fn drop(&mut self) {
        let mut bytes = std::mem::ManuallyDrop::new(std::mem::take(&mut self.bytes));
        // Dealloc manually with initial memory layout.
        if bytes.capacity() != 0 {
            unsafe { std::alloc::dealloc(bytes.as_mut_ptr(), self.layout) }
        }
    }
}
