use std::borrow::Cow;

use crate::{
    frame_graph::PassContext,
    gfx_base::{BindGroupDescriptor, GpuBindGroup, GpuBindGroupLayout},
};

use super::BindGroupEntryInfo;

#[derive(Clone)]
pub struct BindGroupInfo {
    pub label: Option<Cow<'static, str>>,
    pub layout: GpuBindGroupLayout,
    pub entries: Vec<BindGroupEntryInfo>,
}

pub struct BindGroup(GpuBindGroup);

impl BindGroup {
    pub fn get_gpu_bind_group(&self) -> &GpuBindGroup {
        &self.0
    }

    pub fn new(context: &PassContext<'_>, info: &BindGroupInfo) -> Self {
        let entries = info
            .entries
            .iter()
            .map(|entry| entry.get_gpu_bind_group_entry(context))
            .collect();

        let desc = BindGroupDescriptor {
            label: info.label.clone(),
            layout: info.layout.clone(),
            entries,
        };

        BindGroup(context.render_device.create_bind_group(&desc))
    }
}
