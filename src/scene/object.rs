use downcast_rs::{Downcast, impl_downcast};

pub trait DynObject: 'static + Downcast {}

pub struct Object(Box<dyn DynObject>);

impl Object {
    pub fn new<T: DynObject>(value: T) -> Self {
        Object(Box::new(value))
    }

    pub fn cast<T: DynObject>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }

    pub fn cast_mut<T: DynObject>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut()
    }
}

impl_downcast!(DynObject);
