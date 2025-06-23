use crate::{frame_graph::BufferInfo, resource::RenderDevice};
use fyrox_core::log::Log;

use wgpu::{
    BufferUsages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{Buffer, FrameworkError, Geometry, GeometryResource, TemporaryCache};

pub struct GeometryData {
    pub vertex_buffer: Buffer,
}

impl GeometryData {
    pub fn new(device: &RenderDevice, geometry: &Geometry) -> Result<Self, FrameworkError> {
        let bytes = geometry.vertex.create_vertex_data();

        let buffer = device
            .wgpu_device()
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &bytes,
                usage: BufferUsages::VERTEX,
            });

        let buffer_info = BufferInfo {
            label: None,
            size: bytes.len() as u64,
            mapped_at_creation: false,
            usage: BufferUsages::VERTEX,
        };

        let key = format!("vertex_buffer_{}", geometry.cache_index.get());

        let vertex_buffer = Buffer {
            key,
            value: buffer,
            desc: buffer_info,
        };

        Ok(GeometryData { vertex_buffer })
    }
}

#[derive(Default)]
pub struct GeometryStorage {
    pub geometry_cache: TemporaryCache<GeometryData>,
}

impl GeometryStorage {
    pub fn get(
        &mut self,
        device: &RenderDevice,
        geometry: &GeometryResource,
    ) -> Option<&GeometryData> {
        let mut geometry_state = geometry.state();

        if let Some(geometry_state) = geometry_state.data() {
            match self.geometry_cache.get_mut_or_insert_with(
                &geometry_state.cache_index,
                Default::default(),
                || GeometryData::new(device, geometry_state),
            ) {
                Ok(data) => Some(data),
                Err(error) => {
                    Log::err(format!("{error}"));
                    None
                }
            }
        } else {
            None
        }
    }
}
