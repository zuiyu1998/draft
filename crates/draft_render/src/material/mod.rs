mod bind_group;

mod effect;

pub use bind_group::*;
pub use effect::*;

pub struct Material {
    pub name: String,
    pub info: MaterialInfo,
}

impl Material {
    pub fn new<M: IMaterial>() -> Self {
        Self { name: M::name(), info: M::material_info() }
    }
}

pub struct MaterialInfo {
    pub effect_name: Option<String>,
    pub technique: usize,
}

pub trait IMaterial {
    fn name() -> String;

    fn material_info() -> MaterialInfo;
}
