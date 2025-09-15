use super::{
    CompareFunction, RawAddressMode, RawFilterMode, RawSamplerBorderColor, WgpuSampler,
    WgpuSamplerDescriptor,
};
use fyrox_core::{reflect::*, visitor::*};
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Visit, Reflect, Default, Deserialize, Serialize,
)]
pub enum SamplerBorderColor {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    #[default]
    Zero,
}

impl From<SamplerBorderColor> for RawSamplerBorderColor {
    fn from(value: SamplerBorderColor) -> Self {
        match value {
            SamplerBorderColor::OpaqueBlack => RawSamplerBorderColor::OpaqueBlack,
            SamplerBorderColor::TransparentBlack => RawSamplerBorderColor::TransparentBlack,
            SamplerBorderColor::OpaqueWhite => RawSamplerBorderColor::OpaqueWhite,
            SamplerBorderColor::Zero => RawSamplerBorderColor::Zero,
        }
    }
}

impl From<RawSamplerBorderColor> for SamplerBorderColor {
    fn from(value: RawSamplerBorderColor) -> Self {
        match value {
            RawSamplerBorderColor::OpaqueBlack => SamplerBorderColor::OpaqueBlack,
            RawSamplerBorderColor::TransparentBlack => SamplerBorderColor::TransparentBlack,
            RawSamplerBorderColor::OpaqueWhite => SamplerBorderColor::OpaqueWhite,
            RawSamplerBorderColor::Zero => SamplerBorderColor::Zero,
        }
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Reflect, Visit, Deserialize, Serialize,
)]
pub enum FilterMode {
    #[default]
    Nearest = 0,
    Linear = 1,
}

impl From<FilterMode> for RawFilterMode {
    fn from(value: FilterMode) -> Self {
        match value {
            FilterMode::Linear => RawFilterMode::Linear,
            FilterMode::Nearest => RawFilterMode::Nearest,
        }
    }
}

impl From<RawFilterMode> for FilterMode {
    fn from(value: RawFilterMode) -> Self {
        match value {
            RawFilterMode::Linear => FilterMode::Linear,
            RawFilterMode::Nearest => FilterMode::Nearest,
        }
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Reflect, Visit, Deserialize, Serialize,
)]
pub enum AddressMode {
    #[default]
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
    ClampToBorder = 3,
}

impl From<AddressMode> for RawAddressMode {
    fn from(value: AddressMode) -> Self {
        match value {
            AddressMode::ClampToBorder => RawAddressMode::ClampToBorder,
            AddressMode::ClampToEdge => RawAddressMode::ClampToEdge,
            AddressMode::Repeat => RawAddressMode::Repeat,
            AddressMode::MirrorRepeat => RawAddressMode::MirrorRepeat,
        }
    }
}

impl From<RawAddressMode> for AddressMode {
    fn from(value: RawAddressMode) -> Self {
        match value {
            RawAddressMode::ClampToBorder => AddressMode::ClampToBorder,
            RawAddressMode::ClampToEdge => AddressMode::ClampToEdge,
            RawAddressMode::Repeat => AddressMode::Repeat,
            RawAddressMode::MirrorRepeat => AddressMode::MirrorRepeat,
        }
    }
}

#[derive(Debug, Clone, Reflect, Visit, Deserialize, Serialize)]
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
    pub fn get_desc<'a>(&'a self) -> WgpuSamplerDescriptor<'a> {
        WgpuSamplerDescriptor {
            label: self.label.as_deref(),
            address_mode_u: self.address_mode_u.into(),
            address_mode_v: self.address_mode_v.into(),
            address_mode_w: self.address_mode_w.into(),
            mag_filter: self.mag_filter.into(),
            min_filter: self.min_filter.into(),
            mipmap_filter: self.mipmap_filter.into(),
            lod_min_clamp: self.lod_min_clamp,
            lod_max_clamp: self.lod_max_clamp,
            compare: self.compare.map(|compare| compare.into()),
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color.map(|border_color| border_color.into()),
        }
    }
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        SamplerDescriptor {
            label: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

#[derive(Clone)]
pub struct GpuSampler(WgpuSampler);

impl GpuSampler {
    pub fn new(sampler: WgpuSampler) -> Self {
        GpuSampler(sampler)
    }

    pub fn get_sampler(&self) -> &WgpuSampler {
        &self.0
    }
}
