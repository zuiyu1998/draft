mod bind_group;
mod bind_group_layout;
mod buffer;
mod color_target_state;
mod common;
mod depth_stencil;
mod device;
mod multisample_state;
mod pipeline;
mod pipeline_layout;
mod primitive_state;
mod sampler;
mod texture;
mod vertex_buffer_layout;

pub use bind_group::*;
pub use bind_group_layout::*;
pub use buffer::*;
pub use color_target_state::*;
pub use common::*;
pub use depth_stencil::*;
pub use device::*;
pub use multisample_state::*;
pub use pipeline::*;
pub use pipeline_layout::*;
pub use primitive_state::*;
pub use sampler::*;
pub use texture::*;
pub use vertex_buffer_layout::*;

pub use wgpu::{
    AddressMode as RawAddressMode, AstcBlock as RawAstcBlock, AstcChannel as RawAstcChannel,
    BindGroup as WgpuBindGroup, BindGroupDescriptor as WgpuBindGroupDescriptor, BindGroupEntry,
    BindGroupEntry as WgpuBindGroupEntry, BindGroupLayout as WgpuBindGroupLayout,
    BindGroupLayoutDescriptor as RawBindGroupLayoutDescriptor,
    BindGroupLayoutEntry as WgpuBindGroupLayoutEntry, BindingResource as WgpuBindingResource,
    BindingType as RawBindingType, BlendComponent as RawBlendComponent,
    BlendFactor as RawBlendFactor, BlendOperation as RawBlendOperation,
    BlendState as RawBlendState, Buffer as WgpuBuffer, BufferAddress,
    BufferBinding as WgpuBufferBinding, BufferBindingType as RawBufferBindingType,
    BufferDescriptor as RawBufferDescriptor, BufferSize, BufferUsages, COPY_BUFFER_ALIGNMENT,
    Color, ColorTargetState as RawColorTargetState, ColorWrites as RawColorWrites,
    CommandBuffer as RawCommandBuffer, CommandBuffer as WgpuCommandBuffer, CommandEncoder,
    CompareFunction as RawCompareFunction, DepthBiasState as RawDepthBiasState,
    DepthStencilState as RawDepthStencilState, Device as RawDevice, Extent3d as RawExtent3d,
    Face as RawFace, FilterMode as RawFilterMode, FragmentState as RawFragmentState,
    FrontFace as RawFrontFace, IndexFormat as RawIndexFormat, LoadOp,
    MultisampleState as RawMultisampleState, Operations,
    PipelineCompilationOptions as RawPipelineCompilationOptions,
    PipelineLayout as RawPipelineLayout, PipelineLayoutDescriptor as RawPipelineLayoutDescriptor,
    PolygonMode as RawPolygonMode, PrimitiveState as RawPrimitiveState,
    PrimitiveTopology as RawPrimitiveTopology, Queue as WgpuQueue, QueueWriteBufferView,
    RenderPipelineDescriptor as RawRenderPipelineDescriptor, Sampler as WgpuSampler,
    SamplerBindingType as RawSamplerBindingType, SamplerBorderColor as RawSamplerBorderColor,
    SamplerDescriptor as RawSamplerDescriptor, ShaderLocation,
    ShaderModuleDescriptor as RawShaderModuleDescriptor, ShaderSource as RawShaderSource,
    ShaderStages as RawShaderStages, StencilFaceState as RawStencilFaceState,
    StencilOperation as RawStencilOperation, StencilState as RawStencilState,
    StorageTextureAccess as RawStorageTextureAccess, StoreOp, Texture as WgpuTexture,
    TextureAspect, TextureDescriptor as RawTextureDescriptor,
    TextureDimension as RawTextureDimension, TextureFormat as RawTextureFormat,
    TextureSampleType as RawTextureSampleType, TextureUsages as RawTextureUsages,
    TextureView as WgpuTextureView, TextureViewDimension as RawTextureViewDimension,
    VertexAttribute as RawVertexAttribute, VertexBufferLayout as RawVertexBufferLayout,
    VertexFormat as RawVertexFormat, VertexState as RawVertexState,
    VertexStepMode as RawVertexStepMode,
    util::{BufferInitDescriptor as RawBufferInitDescriptor, TextureDataOrder},
};

use std::sync::Arc;
use tracing::info;

use wgpu::{Instance, RequestAdapterOptions};

#[derive(Clone)]
pub struct RenderQueue(Arc<WgpuQueue>);

impl RenderQueue {
    pub fn wgpu_queue(&self) -> &WgpuQueue {
        &self.0
    }

    pub fn submit(&self, buffers: Vec<WgpuCommandBuffer>) {
        self.wgpu_queue().submit(buffers);
    }

    pub fn write_buffer(&self, buffer: &GpuBuffer, offset: BufferAddress, data: &[u8]) {
        self.0.write_buffer(buffer.get_buffer(), offset, data);
    }

    pub fn write_buffer_with<'a>(
        &'a self,
        buffer: &'a GpuBuffer,
        offset: BufferAddress,
        size: BufferSize,
    ) -> Option<QueueWriteBufferView<'a>> {
        self.0.write_buffer_with(buffer.get_buffer(), offset, size)
    }
}

#[derive(Clone)]
pub struct RenderAdapter(pub Arc<wgpu::Adapter>);

#[derive(Clone)]
pub struct RenderInstance(pub Arc<wgpu::Instance>);

#[derive(Clone)]
pub struct RenderAdapterInfo(pub Arc<wgpu::AdapterInfo>);

pub async fn initialize_resources(
    instance: Instance,
    request_adapter_options: &RequestAdapterOptions<'_, '_>,
) -> (
    RenderDevice,
    RenderQueue,
    RenderAdapter,
    RenderAdapterInfo,
    RenderInstance,
) {
    let adapter = instance
        .request_adapter(request_adapter_options)
        .await
        .expect("Unable to find a GPU! Make sure you have installed required drivers!");

    let adapter_info = adapter.get_info();
    info!("{:?}", adapter_info);

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    (
        RenderDevice::new(device),
        RenderQueue(Arc::new(queue)),
        RenderAdapter(Arc::new(adapter)),
        RenderAdapterInfo(Arc::new(adapter_info)),
        RenderInstance(Arc::new(instance)),
    )
}
