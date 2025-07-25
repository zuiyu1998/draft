use super::{MaterialBindGroupHandle, MaterialResourceBinding, MaterialResourceHandleContainer};
use crate::{FrameworkError, RenderPipelineDescriptor, RenderWorld, gfx_base::CachedPipelineId};
use fxhash::FxHashMap;
use fyrox_core::{ImmutableString, reflect::*, visitor::*};

#[derive(Debug, Clone, Reflect, Visit, Default)]
pub struct ResourceBindings(FxHashMap<ImmutableString, MaterialResourceBinding>);

impl ResourceBindings {
    pub fn get(&self, key: &ImmutableString) -> Option<&MaterialResourceBinding> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: ImmutableString, binding: MaterialResourceBinding) {
        self.0.insert(key, binding);
    }

    pub fn extra(
        &self,
        desc: &RenderPipelineDescriptor,
        render_world: &mut RenderWorld,
    ) -> Result<MaterialRenderData, FrameworkError> {
        let bind_group_layout_descs = desc.layout.get_bind_group_layout_descs();
        let mut bind_group_layouts = vec![];

        for bind_group_layout_desc in bind_group_layout_descs.iter() {
            let bind_group_layout = render_world
                .pipeline_cache
                .get_or_create_bind_group_layout(bind_group_layout_desc)?
                .clone();

            bind_group_layouts.push(bind_group_layout);
        }

        let name_containers = desc.layout.get_bind_group_layout_names();
        let mut bind_group_handles = vec![];

        for (index, bind_group_layout) in bind_group_layouts.into_iter().enumerate() {
            let handle_container = MaterialResourceHandleContainer::extra(
                &name_containers[index],
                self,
                render_world,
            )?;

            bind_group_handles.push(MaterialBindGroupHandle {
                bind_group_layout,
                material_resource_handle_container: handle_container,
            });
        }

        let pipeline_id = render_world
            .pipeline_cache
            .get_or_create_with_render_descriptor(desc);

        Ok(MaterialRenderData {
            pipeline_id,
            bind_group_handles,
        })
    }
}

pub struct MaterialRenderData {
    pub pipeline_id: CachedPipelineId,
    pub bind_group_handles: Vec<MaterialBindGroupHandle>,
}
