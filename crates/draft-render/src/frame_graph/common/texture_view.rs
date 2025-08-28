use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, ResourceView, ResourceWrite, TransientTexture},
    gfx_base::{GpuTextureView, TextureViewDescriptor},
};

pub type TextureViewInfoRead = TextureViewInfo<ResourceRead>;

pub type TextureViewInfoWrite = TextureViewInfo<ResourceWrite>;

pub struct TextureViewInfo<ViewType: ResourceView> {
    pub texture: Ref<TransientTexture, ViewType>,
    pub desc: TextureViewDescriptor,
}

impl<ViewType: ResourceView> Clone for TextureViewInfo<ViewType> {
    fn clone(&self) -> Self {
        Self {
            texture: self.texture.clone(),
            desc: self.desc.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TextureView(GpuTextureView);

impl TextureView {
    pub fn new(texture_view: GpuTextureView) -> Self {
        Self(texture_view)
    }

    pub fn get_gpu_texture_view(&self) -> &GpuTextureView {
        &self.0
    }

    pub fn from_info<View: ResourceView>(
        context: &PassContext<'_>,
        info: &TextureViewInfo<View>,
    ) -> Self {
        TextureView::new(
            context
                .resource_table
                .get_resource(&info.texture)
                .resource
                .create_view(&info.desc),
        )
    }
}
