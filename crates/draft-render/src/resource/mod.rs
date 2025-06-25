mod bind_group_layout;
mod buffer;
mod pipeline;
mod pipeline_cache;
mod texture;

use std::sync::Arc;

pub use bind_group_layout::*;
pub use buffer::*;
pub use pipeline::*;
pub use pipeline_cache::*;
pub use texture::*;

use tracing::info;

use wgpu::{Instance, RequestAdapterOptions};

use crate::gfx_base::{RawDevice, RawQueue};

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
