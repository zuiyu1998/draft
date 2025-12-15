use std::{collections::HashMap, sync::Arc};

use draft_graphics::frame_graph::gfx_base::{
    BindGroupLayout, BindGroupLayoutId, PipelineLayout, PipelineLayoutDescriptor, RenderDevice,
};
use wgpu::PushConstantRange;

pub type LayoutCacheKey = (Vec<BindGroupLayoutId>, Vec<PushConstantRange>);

#[derive(Default)]
pub struct LayoutCache {
    layouts: HashMap<LayoutCacheKey, Arc<PipelineLayout>>,
}

impl LayoutCache {
    pub fn get(
        &mut self,
        render_device: &RenderDevice,
        bind_group_layouts: &[BindGroupLayout],
        push_constant_ranges: Vec<PushConstantRange>,
    ) -> Arc<PipelineLayout> {
        let bind_group_ids = bind_group_layouts.iter().map(BindGroupLayout::id).collect();
        self.layouts
            .entry((bind_group_ids, push_constant_ranges))
            .or_insert_with_key(|(_, push_constant_ranges)| {
                let bind_group_layouts = bind_group_layouts
                    .iter()
                    .map(|bind_group_layout| bind_group_layout.value().clone())
                    .collect::<Vec<_>>();
                Arc::new(PipelineLayout::new(render_device.create_pipeline_layout(
                    &PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts,
                        push_constant_ranges: push_constant_ranges.to_vec(),
                    },
                )))
            })
            .clone()
    }
}
