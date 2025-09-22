use std::ops::{Deref, DerefMut};

use draft_render::{
    RenderWorld,
    frame_graph::{FrameGraph, TextureView},
    render_resource::RenderBuffer,
};
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::renderer::{CameraUniforms, RenderDataBundleStorage};

pub struct RenderPipelineContainer(FxHashMap<ImmutableString, RenderPipeline>);

impl Default for RenderPipelineContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelineContainer {
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn new() -> Self {
        RenderPipelineContainer::empty()
    }
}

impl Deref for RenderPipelineContainer {
    type Target = FxHashMap<ImmutableString, RenderPipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderPipelineContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct CameraContext {
    uniforms: CameraUniforms,
    index: Option<usize>,
}

pub struct FrameGraphContext<'a> {
    camera: Option<CameraContext>,
    pub texture_view: TextureView,
    pub render_data_bundle_storage: &'a RenderDataBundleStorage,
}

impl<'a> FrameGraphContext<'a> {
    pub fn new(
        render_data_bundle_storage: &'a RenderDataBundleStorage,
        texture_view: TextureView,
    ) -> Self {
        Self {
            camera: None,
            texture_view,
            render_data_bundle_storage,
        }
    }

    pub fn alloc_camera_buffer(&mut self, render_world: &mut RenderWorld) {
        let camera_uniforms = self
            .render_data_bundle_storage
            .get_camera_uniforms(render_world)
            .unwrap();
        self.camera = Some(CameraContext {
            uniforms: camera_uniforms,
            index: None,
        })
    }

    pub fn get_camera_buffer(&self) -> RenderBuffer {
        self.camera
            .as_ref()
            .map(|v| v.uniforms.get_camera_buffer())
            .unwrap()
    }

    pub fn set_camera(&mut self, index: usize) {
        if let Some(ref mut camera) = self.camera {
            camera.index = Some(index);
        }
    }
}

pub trait RenderPipelineNode: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
    );
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn RenderPipelineNode>>,
}

impl RenderPipeline {
    pub fn empty() -> Self {
        RenderPipeline { nodes: vec![] }
    }

    pub fn push_node<T: RenderPipelineNode>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, frame_graph_context);
        }
    }
}
