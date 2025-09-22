#[allow(dead_code)]
mod camera;
mod mesh;
mod observer;

pub use camera::*;
use encase::ShaderType;
pub use mesh::*;
pub use observer::*;

use draft_render::{DynamicUniformBuffer, RenderWorld, render_resource::RenderBuffer};

pub struct FrameContext {
    pub render_data_bundle_storage: RenderDataBundleStorage,
    pub camera_uniforms: Option<CameraUniforms>,
}

impl FrameContext {
    pub fn new(
        render_data_bundle_storage: RenderDataBundleStorage,
        render_world: &mut RenderWorld,
    ) -> Self {
        let camera_uniforms = render_data_bundle_storage.get_camera_uniforms(render_world);
        FrameContext {
            render_data_bundle_storage,
            camera_uniforms,
        }
    }

    pub fn get_camera_uniforms(&self) -> Option<&CameraUniforms> {
        self.camera_uniforms.as_ref()
    }

    pub fn get_camera_buffer(&self) -> RenderBuffer {
        self.camera_uniforms
            .as_ref()
            .map(|v| v.get_camera_buffer())
            .expect("camera uniforms must have")
    }
}

pub struct RenderDataBundleStorage {
    pub mesh_render_data_bundle_storage: Box<dyn MeshRenderDataBundleStorage>,
    pub cameras: Vec<Observer>,
}

impl RenderDataBundleStorage {
    pub fn get_camera_uniforms(&self, render_world: &mut RenderWorld) -> Option<CameraUniforms> {
        if self.cameras.is_empty() {
            return None;
        }

        let mut buffer = DynamicUniformBuffer::<CameraUniform>::default();
        let mut offsets = vec![];

        {
            let mut writer = buffer
                .get_writer(
                    self.cameras.len(),
                    &render_world.server.device,
                    &render_world.server.queue,
                )
                .unwrap();

            for camera in self.cameras.iter() {
                offsets.push(writer.write(&CameraUniform {
                    view_projection_matrix: camera.position.view_projection_matrix,
                }));
            }
        }

        let buffer = buffer.into_inner().unwrap();

        let size = CameraUniform::min_size();

        Some(CameraUniforms::new(offsets, size, buffer))
    }

    pub fn new<T>(cameras: Vec<Observer>, mesh_render_data_bundle_storage: T) -> Self
    where
        T: MeshRenderDataBundleStorage,
    {
        Self {
            mesh_render_data_bundle_storage: Box::new(mesh_render_data_bundle_storage),
            cameras,
        }
    }
}
