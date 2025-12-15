use wgpu::{
    Extent3d, SurfaceTexture as WgpuSurfaceTexture, Texture as WgpuTexture,
    TextureDescriptor as WgpuTextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

use super::{GpuTextureView, TextureViewDescriptor};

pub struct GpuSurfaceTexture(WgpuSurfaceTexture);

impl GpuSurfaceTexture {
    pub(crate) fn new(value: WgpuSurfaceTexture) -> Self {
        GpuSurfaceTexture(value)
    }

    pub fn present(self) {
        self.0.present();
    }
}

#[derive(Debug, Clone)]
pub struct GpuTexture(WgpuTexture);

impl GpuTexture {
    pub fn new(texture: WgpuTexture) -> Self {
        Self(texture)
    }

    pub fn create_view(&self, desc: &TextureViewDescriptor) -> GpuTextureView {
        GpuTextureView::new(self.0.create_view(&desc.get_desc()))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TextureDescriptor {
    pub label: Option<String>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: Vec<TextureFormat>,
}

impl TextureDescriptor {
    pub fn get_wgpu_desc(&self) -> WgpuTextureDescriptor<'_> {
        WgpuTextureDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: &self.view_formats,
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub value: GpuTexture,
    pub desc: TextureDescriptor,
}
