use crate::{frame_graph::BufferInfo, gfx_base::RenderDevice};
use fyrox_core::log::Log;

use wgpu::{
    BufferUsages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    FrameworkError, Geometry, GeometryResource, TemporaryCache, render_resource::RenderBuffer,
};

pub struct GeometryData {
    pub vertex_buffer: RenderBuffer,
}

fn get_vertex_buffer_key(geometry: &Geometry) -> String {
    format!("vertex_buffer_{}", geometry.cache_index.get())
}

impl GeometryData {
    pub fn update(&mut self, geometry: &Geometry) {
        self.vertex_buffer.key = get_vertex_buffer_key(geometry);
    }

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

        let key = get_vertex_buffer_key(geometry);

        let vertex_buffer = RenderBuffer {
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
                Ok(data) => {
                    data.update(geometry_state);
                    Some(data)
                }
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
