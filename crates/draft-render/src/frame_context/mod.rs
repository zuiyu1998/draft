#[allow(dead_code)]
mod camera;
mod mesh;

pub use camera::*;
pub use mesh::*;

use crate::{GeometryResource, MaterialResource};

pub trait RenderDataBundleStorage: 'static {
    fn push_mesh(
        &mut self,
        geometry: GeometryResource,
        material: MaterialResource,
        sort_index: u64,
        instance_data: MeshInstanceData,
    );
}

pub struct FrameContext {
    pub render_data_bundle_storage: Box<dyn RenderDataBundleStorage>,
    pub camera_uniforms: CameraUniforms,
}

impl FrameContext {
    pub fn new<T>(camera_uniforms: CameraUniforms, render_data_bundle_storage: T) -> Self
    where
        T: RenderDataBundleStorage,
    {
        Self {
            render_data_bundle_storage: Box::new(render_data_bundle_storage),
            camera_uniforms,
        }
    }
}
