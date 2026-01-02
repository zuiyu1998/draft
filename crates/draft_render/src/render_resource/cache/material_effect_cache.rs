use draft_graphics::gfx_base::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutId, RenderDevice,
};
use draft_material::{MaterialEffect, MaterialEffectResource};
use fxhash::FxHashMap;
use fyrox_resource::{event::ResourceEvent, manager::ResourceManager};
use std::{
    collections::hash_map::Entry,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
};

use crate::CacheEntry;

pub struct MaterialEffectData {
    pub key: u64,
    modifications_counter: u64,
    instance: MaterialEffectInstance,
}

#[derive(Default)]
pub struct BindGroupLayoutCache {
    desc_to_id: FxHashMap<BindGroupLayoutDescriptor, BindGroupLayoutId>,
    cache: FxHashMap<BindGroupLayoutId, Arc<BindGroupLayout>>,
}

impl BindGroupLayoutCache {
    pub fn get_bind_group_layout(
        &mut self,
        desc: BindGroupLayoutDescriptor,
        render_device: &RenderDevice,
    ) -> Arc<BindGroupLayout> {
        if let Some(id) = self.desc_to_id.get(&desc) {
            self.cache.get(id).unwrap().clone()
        } else {
            let bind_group_layout =
                BindGroupLayout::new(render_device.create_bind_group_layout(&desc));
            let id = bind_group_layout.id();

            let bind_group_layout = Arc::new(bind_group_layout);

            self.desc_to_id.insert(desc, id);
            self.cache.insert(id, bind_group_layout.clone());

            bind_group_layout
        }
    }
}

pub struct MaterialEffectCache {
    render_device: RenderDevice,
    cache: FxHashMap<u64, CacheEntry<MaterialEffectData>>,
    effect_name_to_cache: FxHashMap<String, u64>,
    data: FxHashMap<u64, MaterialEffectInstance>,
    bind_group_layout_cache: BindGroupLayoutCache,
    material_effect_receiver: Receiver<ResourceEvent>,
}

impl MaterialEffectCache {
    pub fn new(render_device: RenderDevice, resource_manager: &ResourceManager) -> Self {
        let (rx, tx) = channel();

        resource_manager.state().event_broadcaster.add(rx);

        Self {
            render_device,
            cache: Default::default(),
            effect_name_to_cache: Default::default(),
            data: Default::default(),
            bind_group_layout_cache: Default::default(),
            material_effect_receiver: tx,
        }
    }

    pub fn set_material_effect(&mut self, material_effect: MaterialEffectResource) {
        let key = material_effect.key();
        let material_effect = material_effect.data_ref();

        match self.cache.entry(key) {
            Entry::Occupied(mut entry) => {
                if entry.get().value.modifications_counter
                    != material_effect.modifications_counter()
                {
                    let instance = create_material_effect_instance(
                        &mut self.bind_group_layout_cache,
                        &self.render_device,
                        &material_effect,
                    );

                    entry.get_mut().value.instance = instance;
                }
            }
            Entry::Vacant(entry) => {
                let instance = create_material_effect_instance(
                    &mut self.bind_group_layout_cache,
                    &self.render_device,
                    &material_effect,
                );

                entry.insert(CacheEntry::new(MaterialEffectData {
                    key,
                    modifications_counter: material_effect.modifications_counter(),
                    instance,
                }));
            }
        }
    }

    pub fn get_material_effect_instance(
        &self,
        effect_name: &str,
    ) -> Option<&MaterialEffectInstance> {
        self.effect_name_to_cache
            .get(effect_name)
            .and_then(|key| self.data.get(key))
    }

    pub fn get_material_technique_instance(
        &self,
        effect_name: &str,
        technique: usize,
    ) -> Option<&MaterialTechniqueInstance> {
        self.get_material_effect_instance(effect_name)
            .and_then(|instance| instance.techniques.get(technique))
    }

    fn process(&mut self, dt: f32) {
        let mut removed = vec![];

        for data in self.cache.values_mut() {
            data.update(dt);
            if data.free() {
                removed.push(data.value.key);
            }
        }

        for removed in removed.into_iter() {
            self.data.remove(&removed);
        }
    }

    fn handle_material_effect_event(&mut self) {
        while let Ok(event) = self.material_effect_receiver.try_recv() {
            if let ResourceEvent::Loaded(resource) | ResourceEvent::Reloaded(resource) = event {
                if let Some(material_effect) = resource.try_cast::<MaterialEffect>() {
                    self.set_material_effect(material_effect);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.handle_material_effect_event();

        self.process(dt);
    }
}

pub struct MaterialEffectInstance {
    techniques: Vec<MaterialTechniqueInstance>,
}

pub struct MaterialTechniqueInstance {
    pub name: String,
    pub bind_groups: Vec<MaterialBindGroupInstance>,
}

pub struct MaterialBindGroupInstance {
    pub name: String,
    pub bind_group_layouts: Vec<MaterialBindGroupLayoutInstance>,
}

pub struct MaterialBindGroupLayoutInstance {
    pub name: String,
    pub bind_group_layout: Arc<BindGroupLayout>,
    pub resource_bindings: Vec<String>,
}

pub fn create_material_effect_instance(
    bind_group_layout_cache: &mut BindGroupLayoutCache,
    render_device: &RenderDevice,
    material_effect: &MaterialEffect,
) -> MaterialEffectInstance {
    let mut techniques = vec![];
    for technique in material_effect.techniques.iter() {
        let mut bind_groups = vec![];

        for bind_group in technique.bind_groups.iter() {
            let mut bind_group_layouts = vec![];

            for bind_group_layout in bind_group.layouts.iter() {
                let mut resource_bindings = vec![];
                let mut entries = vec![];

                bind_group_layout.entries.iter().for_each(|entry| {
                    entries.push(entry.get_bind_group_layout_entry());
                    resource_bindings.push(entry.name.clone());
                });

                let desc = BindGroupLayoutDescriptor {
                    label: Some(bind_group_layout.name.clone()),
                    entries,
                };

                let bind_group_layout_instance =
                    bind_group_layout_cache.get_bind_group_layout(desc, render_device);

                bind_group_layouts.push(MaterialBindGroupLayoutInstance {
                    bind_group_layout: bind_group_layout_instance,
                    name: bind_group_layout.name.clone(),
                    resource_bindings,
                });
            }

            bind_groups.push(MaterialBindGroupInstance {
                bind_group_layouts,
                name: bind_group.name.clone(),
            });
        }

        techniques.push(MaterialTechniqueInstance {
            bind_groups,
            name: technique.name.clone(),
        });
    }

    MaterialEffectInstance { techniques }
}
