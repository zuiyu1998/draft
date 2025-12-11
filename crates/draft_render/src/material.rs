pub struct MaterialInstance {
    pub name: String,
}

impl MaterialInstance {
    pub fn new<M: Material>() -> Self {
        Self { name: M::name() }
    }
}

pub trait Material {
    fn name() -> String;
}