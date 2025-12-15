use crate::{
    frame_graph::{PassContext, Ref, ResourceRead, ResourceView, ResourceWrite, TransientTexture},
    gfx_base::{GpuTextureView, TextureView, TextureViewDescriptor},
};

pub type TransientTextureViewDescriptorRead = TransientTextureViewDescriptor<ResourceRead>;

pub type TransientTextureViewDescriptorWrite = TransientTextureViewDescriptor<ResourceWrite>;

pub struct TransientTextureViewDescriptor<ViewType> {
    pub texture: Ref<TransientTexture, ViewType>,
    pub desc: TextureViewDescriptor,
}

impl<ViewType: ResourceView> TransientTextureViewDescriptor<ViewType> {
    pub fn create_gpu_texture_view(&self, context: &PassContext) -> GpuTextureView {
        let resource = context.get_resource(&self.texture);
        resource.resource.create_view(&self.desc)
    }
}

pub enum TransientTextureView {
    Read(TransientTextureViewDescriptorRead),
    Write(TransientTextureViewDescriptorWrite),
    Owned(TextureView),
}

impl TransientTextureView {
    pub fn create_gpu_texture_view(&self, context: &PassContext) -> GpuTextureView {
        match self {
            TransientTextureView::Read(desc) => desc.create_gpu_texture_view(context),
            TransientTextureView::Write(desc) => desc.create_gpu_texture_view(context),
            TransientTextureView::Owned(texture_view) => texture_view.value().clone(),
        }
    }
}
