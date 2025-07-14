use fxhash::{FxHashMap, FxHasher};
use std::{
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{
    MaterialDefinition, MaterialResource, PipelineDescriptor, gfx_base::VertexBufferLayout,
};

#[derive(Default)]
pub struct PipelineDescriptorCache(FxHashMap<u64, PipelineDescriptorData>);

pub struct PipelineDescriptorData {
    desc: PipelineDescriptor,
    modifications_counter: u64,
}

fn create_pipeline_descriptor(
    layouts: &[VertexBufferLayout],
    material: &MaterialDefinition,
) -> PipelineDescriptor {
    let mut desc = PipelineDescriptor::default();
    {
        let desc = desc.render_pipeline_descriptor().unwrap();
        desc.vertex.buffers = layouts.to_vec();
    }

    material.specialize(&mut desc);

    desc
}

impl PipelineDescriptorCache {
    pub fn get_or_create(
        &mut self,
        layouts: &[VertexBufferLayout],
        material: &MaterialResource,
    ) -> Option<&PipelineDescriptor> {
        let mut hasher = FxHasher::default();
        layouts.hash(&mut hasher);
        material.key().hash(&mut hasher);
        let key = hasher.finish();

        if let Some(data) = self.0.get_mut(&key) {
            let material_state = material.state();

            if let Some(material_state) = material_state.data_ref() {
                if data.modifications_counter != material_state.modifications_counter {
                    data.modifications_counter = material_state.modifications_counter;

                    data.desc = create_pipeline_descriptor(layouts, &material_state.definition)
                };
            }
        } else {
            let material_state = material.state();

            if let Some(material_state) = material_state.data_ref() {
                let desc = create_pipeline_descriptor(layouts, &material_state.definition);

                self.0.insert(
                    key,
                    PipelineDescriptorData {
                        desc,
                        modifications_counter: material_state.modifications_counter,
                    },
                );
            }
        }

        self.get(&key).map(|data| &data.desc)
    }
}

impl Deref for PipelineDescriptorCache {
    type Target = FxHashMap<u64, PipelineDescriptorData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
