use wgpu_types::Extent3d;

use crate::{
    AddressMode, CompareFunction, FilterMode, MipmapFilterMode, SamplerBorderColor,
    SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

pub fn convert_address_mode(mode: &AddressMode) -> wgpu_types::AddressMode {
    match mode {
        AddressMode::ClampToBorder => wgpu_types::AddressMode::ClampToBorder,
        AddressMode::ClampToEdge => wgpu_types::AddressMode::ClampToEdge,
        AddressMode::MirrorRepeat => wgpu_types::AddressMode::MirrorRepeat,
        AddressMode::Repeat => wgpu_types::AddressMode::Repeat,
    }
}

pub fn convert_filter_mode(mode: &FilterMode) -> wgpu_types::FilterMode {
    match mode {
        FilterMode::Linear => wgpu_types::FilterMode::Linear,
        FilterMode::Nearest => wgpu_types::FilterMode::Nearest,
    }
}

pub fn convert_mipmap_filter_mode(mode: &MipmapFilterMode) -> wgpu_types::MipmapFilterMode {
    match mode {
        MipmapFilterMode::Linear => wgpu_types::MipmapFilterMode::Linear,
        MipmapFilterMode::Nearest => wgpu_types::MipmapFilterMode::Nearest,
    }
}

pub fn convert_compare_function(compare: &CompareFunction) -> wgpu_types::CompareFunction {
    match compare {
        CompareFunction::Never => wgpu_types::CompareFunction::Never,
        CompareFunction::Less => wgpu_types::CompareFunction::Less,
        CompareFunction::Equal => wgpu_types::CompareFunction::Greater,
        CompareFunction::LessEqual => wgpu_types::CompareFunction::LessEqual,
        CompareFunction::Greater => wgpu_types::CompareFunction::Greater,
        CompareFunction::NotEqual => wgpu_types::CompareFunction::NotEqual,
        CompareFunction::GreaterEqual => wgpu_types::CompareFunction::GreaterEqual,
        CompareFunction::Always => wgpu_types::CompareFunction::Always,
    }
}

pub fn convert_sampler_border_color(
    border_color: &SamplerBorderColor,
) -> wgpu_types::SamplerBorderColor {
    match border_color {
        SamplerBorderColor::TransparentBlack => wgpu_types::SamplerBorderColor::TransparentBlack,
        SamplerBorderColor::OpaqueBlack => wgpu_types::SamplerBorderColor::OpaqueBlack,
        SamplerBorderColor::OpaqueWhite => wgpu_types::SamplerBorderColor::OpaqueWhite,
        SamplerBorderColor::Zero => wgpu_types::SamplerBorderColor::Zero,
    }
}

pub fn convert_sampler_descriptor(
    desc: &SamplerDescriptor,
) -> wgpu_types::SamplerDescriptor<Option<&str>> {
    let desc = wgpu_types::SamplerDescriptor {
        label: desc.label.as_deref(),
        address_mode_u: convert_address_mode(&desc.address_mode_u),
        address_mode_v: convert_address_mode(&desc.address_mode_v),
        address_mode_w: convert_address_mode(&desc.address_mode_w),
        mag_filter: convert_filter_mode(&desc.mag_filter),
        min_filter: convert_filter_mode(&desc.min_filter),
        mipmap_filter: convert_mipmap_filter_mode(&desc.mipmap_filter),
        lod_min_clamp: desc.lod_min_clamp,
        lod_max_clamp: desc.lod_max_clamp,
        compare: desc
            .compare
            .as_ref()
            .map(|compare| convert_compare_function(compare)),
        anisotropy_clamp: desc.anisotropy_clamp,
        border_color: desc
            .border_color
            .as_ref()
            .map(|border_color| convert_sampler_border_color(border_color)),
    };

    desc
}

pub fn convert_texture_dimension(dimension: &TextureDimension) -> wgpu_types::TextureDimension {
    match dimension {
        TextureDimension::D1 => wgpu_types::TextureDimension::D1,
        TextureDimension::D2 => wgpu_types::TextureDimension::D2,
        TextureDimension::D3 => wgpu_types::TextureDimension::D3,
    }
}

pub fn convert_texture_format(format: &TextureFormat) -> wgpu_types::TextureFormat {
    match format {
        TextureFormat::R16Uint => wgpu_types::TextureFormat::R16Uint,
        TextureFormat::R8Unorm => wgpu_types::TextureFormat::R8Unorm,
        TextureFormat::Rg16Uint => wgpu_types::TextureFormat::Rg16Uint,
        TextureFormat::Rgba16Unorm => wgpu_types::TextureFormat::Rgba16Unorm,
        TextureFormat::Rgba32Float => wgpu_types::TextureFormat::Rgba32Float,
        TextureFormat::Rgba8Unorm => wgpu_types::TextureFormat::Rgba8Unorm,
        TextureFormat::Rgba8UnormSrgb => wgpu_types::TextureFormat::Rgba8UnormSrgb,
    }
}

pub fn convert_texture_usages(usage: &TextureUsages) -> wgpu_types::TextureUsages {
    wgpu_types::TextureUsages::from_bits(usage.bits()).unwrap()
}

pub fn convert_texture_descriptor(
    desc: &TextureDescriptor,
) -> wgpu_types::TextureDescriptor<Option<&str>, Vec<wgpu_types::TextureFormat>> {
    let desc = wgpu_types::TextureDescriptor {
        label: desc.label.as_deref(),
        size: Extent3d {
            width: desc.size.width,
            height: desc.size.height,
            depth_or_array_layers: desc.size.depth_or_array_layers,
        },
        mip_level_count: desc.mip_level_count,
        sample_count: desc.sample_count,
        dimension: convert_texture_dimension(&desc.dimension),
        format: convert_texture_format(&desc.format),
        usage: convert_texture_usages(&desc.usage),
        view_formats: desc
            .view_formats
            .iter()
            .map(|format| convert_texture_format(format))
            .collect::<Vec<_>>(),
    };

    desc
}
