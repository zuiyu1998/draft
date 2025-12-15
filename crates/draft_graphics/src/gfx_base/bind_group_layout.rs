use wgpu::{BindGroupLayout as WgpuBindGroupLayout, BindGroupLayoutEntry};

use crate::define_atomic_id;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Clone, Debug)]
pub struct GpuBindGroupLayout(WgpuBindGroupLayout);

impl GpuBindGroupLayout {
    pub fn new(value: WgpuBindGroupLayout) -> Self {
        GpuBindGroupLayout(value)
    }

    pub(crate) fn get_wgpu_bind_group_layout(&self) -> &WgpuBindGroupLayout {
        &self.0
    }
}

define_atomic_id!(BindGroupLayoutId);

#[derive(Clone, Debug)]
pub struct BindGroupLayout {
    id: BindGroupLayoutId,
    value: GpuBindGroupLayout,
}

impl BindGroupLayout {
    pub fn new(value: GpuBindGroupLayout) -> Self {
        BindGroupLayout {
            value,
            id: BindGroupLayoutId::new(),
        }
    }

    pub fn value(&self) -> &GpuBindGroupLayout {
        &self.value
    }

    pub fn id(&self) -> BindGroupLayoutId {
        self.id
    }
}
