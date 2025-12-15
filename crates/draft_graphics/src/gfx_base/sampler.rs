use wgpu::{
    AddressMode, CompareFunction, FilterMode, Sampler as WgpuSampler, SamplerBorderColor,
    SamplerDescriptor as WgpuSamplerDescriptor,
};

use crate::define_atomic_id;

#[derive(Debug, Clone)]
pub struct SamplerDescriptor {
    pub label: Option<String>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<SamplerBorderColor>,
}

impl SamplerDescriptor {
    pub fn get_wgpu_desc<'a>(&'a self) -> WgpuSamplerDescriptor<'a> {
        WgpuSamplerDescriptor {
            label: self.label.as_deref(),
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            lod_min_clamp: self.lod_min_clamp,
            lod_max_clamp: self.lod_max_clamp,
            compare: self.compare,
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GpuSampler(WgpuSampler);

impl GpuSampler {
    pub fn new(sampler: WgpuSampler) -> Self {
        GpuSampler(sampler)
    }

    pub(crate) fn get_wgpu_sampler(&self) -> &WgpuSampler {
        &self.0
    }
}

define_atomic_id!(SamplerId);

#[derive(Debug, Clone)]
pub struct Sampler {
    id: SamplerId,
    value: GpuSampler,
}

impl PartialEq for Sampler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Sampler {}

impl Sampler {
    pub fn new(value: GpuSampler) -> Self {
        Sampler {
            id: SamplerId::new(),
            value,
        }
    }

    pub fn id(&self) -> SamplerId {
        self.id
    }

    pub fn value(&self) -> &GpuSampler {
        &self.value
    }
}
