use wgpu::{
    TextureAspect, TextureFormat, TextureUsages, TextureView as WgpuTextureView,
    TextureViewDescriptor as WgpuTextureViewDescriptor, TextureViewDimension,
};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct TextureViewDescriptor {
    pub label: Option<String>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub usage: Option<TextureUsages>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}

impl TextureViewDescriptor {
    pub fn get_desc<'a>(&'a self) -> WgpuTextureViewDescriptor<'a> {
        wgpu::TextureViewDescriptor {
            label: self.label.as_deref(),
            format: self.format,
            dimension: self.dimension,
            usage: self.usage,
            aspect: self.aspect,
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count,
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GpuTextureView(WgpuTextureView);

impl GpuTextureView {
    pub fn new(texture_view: WgpuTextureView) -> Self {
        Self(texture_view)
    }

    pub(crate) fn get_wgpu_texture_view(&self) -> &WgpuTextureView {
        &self.0
    }
}

pub struct TextureView {
    value: GpuTextureView,
}

impl TextureView {
    pub(crate) fn value(&self) -> &GpuTextureView {
        &self.value
    }
}
