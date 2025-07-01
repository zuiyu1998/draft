mod bind_group_layout;
mod color_target_state;
mod depth_stencil;
mod multisample_state;
mod pipeline;
mod pipeline_cache;
mod primitive_state;
mod sampler;
mod texture;
mod vertex_buffer_layout;

pub use bind_group_layout::*;
pub use color_target_state::*;
pub use depth_stencil::*;
pub use multisample_state::*;
pub use pipeline::*;
pub use pipeline_cache::*;
pub use primitive_state::*;
pub use sampler::*;
pub use texture::*;
pub use vertex_buffer_layout::*;

pub use wgpu::{
    AddressMode as RawAddressMode, AstcBlock as RawAstcBlock, AstcChannel as RawAstcChannel,
    BindGroupEntry, BindGroupLayoutEntry as RawBindGroupLayoutEntry, BindingType as RawBindingType,
    BlendComponent as RawBlendComponent, BlendFactor as RawBlendFactor,
    BlendOperation as RawBlendOperation, BlendState as RawBlendState, Buffer as RawBuffer,
    BufferAddress, BufferBindingType as RawBufferBindingType, COPY_BUFFER_ALIGNMENT, Color,
    ColorTargetState as RawColorTargetState, ColorWrites as RawColorWrites,
    CompareFunction as RawCompareFunction, DepthBiasState as RawDepthBiasState,
    DepthStencilState as RawDepthStencilState, Device as RawDevice, Extent3d as RawExtent3d,
    Face as RawFace, FilterMode as RawFilterMode, FragmentState as RawFragmentState,
    FrontFace as RawFrontFace, IndexFormat as RawIndexFormat, LoadOp,
    MultisampleState as RawMultisampleState, Operations,
    PipelineCompilationOptions as RawPipelineCompilationOptions, PolygonMode as RawPolygonMode,
    PrimitiveState as RawPrimitiveState, PrimitiveTopology as RawPrimitiveTopology,
    Queue as RawQueue, RenderPipelineDescriptor as RawRenderPipelineDescriptor,
    Sampler as RawSampler, SamplerBindingType as RawSamplerBindingType,
    SamplerBorderColor as RawSamplerBorderColor, SamplerDescriptor as RawSamplerDescriptor,
    ShaderLocation, ShaderModuleDescriptor as RawShaderModuleDescriptor,
    ShaderSource as RawShaderSource, ShaderStages as RawShaderStages,
    StencilFaceState as RawStencilFaceState, StencilOperation as RawStencilOperation,
    StencilState as RawStencilState, StorageTextureAccess as RawStorageTextureAccess, StoreOp,
    Texture as RawTexture, TextureDescriptor as RawTextureDescriptor,
    TextureDimension as RawTextureDimension, TextureFormat as RawTextureFormat,
    TextureSampleType as RawTextureSampleType, TextureUsages as RawTextureUsages,
    TextureView as RawTextureView, TextureViewDimension as RawTextureViewDimension,
    VertexAttribute as RawVertexAttribute, VertexBufferLayout as RawVertexBufferLayout,
    VertexFormat as RawVertexFormat, VertexState as RawVertexState,
    VertexStepMode as RawVertexStepMode,
};

use std::sync::Arc;
use tracing::info;

use wgpu::{Instance, RequestAdapterOptions};

#[derive(Clone)]
pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn wgpu_device(&self) -> &RawDevice {
        &self.device
    }
}

#[derive(Clone)]
pub struct RenderQueue(Arc<RawQueue>);

impl RenderQueue {
    pub fn wgpu_queue(&self) -> &RawQueue {
        &self.0
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
        RenderDevice { device },
        RenderQueue(Arc::new(queue)),
        RenderAdapter(Arc::new(adapter)),
        RenderAdapterInfo(Arc::new(adapter_info)),
        RenderInstance(Arc::new(instance)),
    )
}
