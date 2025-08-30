use std::sync::{Arc, LazyLock};

use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, parking_lot::Mutex};

use crate::MaterialEffect;

static MATERIAL_EFFECT_CONTAINER: LazyLock<MaterialEffectContainer> =
    LazyLock::new(MaterialEffectContainer::default);

#[derive(Default, Clone)]
pub struct MaterialEffectContainer(Arc<Mutex<FxHashMap<ImmutableString, MaterialEffect>>>);

impl MaterialEffectContainer {
    pub fn get_singleton() -> MaterialEffectContainer {
        MATERIAL_EFFECT_CONTAINER.clone()
    }

    pub fn get(&self, name: &ImmutableString) -> Option<MaterialEffect> {
        let guard = self.0.lock();
        guard.get(name).cloned()
    }
    pub fn register_material_effect(&mut self, effect: MaterialEffect) {
        let mut guard = self.0.lock();
        guard.insert(effect.effect_name.clone(), effect);
    }
}
