use alloc::sync::Arc;
use core::{any::Any, marker::PhantomData, ops::Deref};

#[derive(Debug)]
pub struct WindowWrapper<W> {
    reference: Arc<dyn Any + Send + Sync>,
    ty: PhantomData<W>,
}

impl<W: Send + Sync + 'static> WindowWrapper<W> {
    /// Creates a `WindowWrapper` from a window.
    pub fn new(window: W) -> WindowWrapper<W> {
        WindowWrapper {
            reference: Arc::new(window),
            ty: PhantomData,
        }
    }
}

impl<W: 'static> Deref for WindowWrapper<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.reference.downcast_ref::<W>().unwrap()
    }
}
