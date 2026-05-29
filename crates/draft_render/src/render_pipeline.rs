use std::collections::HashMap;

use crate::{
    RenderOptions,
    frame_graph::{FrameGraph, TransientRenderPassColorAttachment, TransientTextureView},
    render_phase::RenderPhase,
    render_world::RenderWorld,
};
use draft_graphics::TextureView;

#[derive(Default)]
pub struct RenderPipelineContainer {
    pipelines: HashMap<String, RenderPipeline>,
}

impl RenderPipelineContainer {
    pub fn insert(&mut self, name: &str, pipeline: RenderPipeline) {
        self.pipelines.insert(name.to_string(), pipeline);
    }

    pub fn get(&self, name: &str) -> Option<&RenderPipeline> {
        self.pipelines.get(name)
    }
}

pub struct RenderPipelineRunContext<'a> {
    pub phases: &'a Vec<RenderPhase>,
    pub(crate) world: &'a mut RenderWorld,
    pub(crate) options: &'a RenderOptions,
}

pub enum RenderTarget {
    Primary,
}

impl RenderPipelineRunContext<'_> {
    pub fn create_target_texture_view(&mut self, render_target: RenderTarget) -> TextureView {
        let handle = match render_target {
            RenderTarget::Primary => self.world.get_primary(),
        };

        let render_window = self.world.get_window_mut(&handle);

        render_window.set_camera();

        let output = render_window.swap_chain_texture();

        output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }
}

pub trait Node {
    fn run(&self, frame_graph: &mut FrameGraph, context: &mut RenderPipelineRunContext);
}

pub struct RenderPipeline {
    nodes: Vec<Box<dyn Node>>,
}

impl RenderPipeline {
    pub fn empty() -> Self {
        Self { nodes: vec![] }
    }

    pub fn push(&mut self, node: impl Node + 'static) {
        self.nodes.push(Box::new(node));
    }

    pub fn push_boxed(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }
}

impl RenderPipeline {
    pub fn run(&self, frame_graph: &mut FrameGraph, context: &mut RenderPipelineRunContext) {
        for node in self.nodes.iter() {
            node.run(frame_graph, context);
        }
    }
}

pub fn initialize_2d_render_pipeline() -> RenderPipeline {
    let mut render_pipeline = RenderPipeline::empty();

    render_pipeline.push(Main2dNode);
    render_pipeline
}

pub struct Main2dNode;

impl Node for Main2dNode {
    fn run(&self, frame_graph: &mut FrameGraph, context: &mut RenderPipelineRunContext) {
        let mut pass_builder = frame_graph.create_pass_builder("main_2d");
        let mut render_pass_builder = pass_builder.create_render_pass_builder("main_2d_node");

        let texture_view = context.create_target_texture_view(RenderTarget::Primary);

        render_pass_builder.add_color_attachment(TransientRenderPassColorAttachment {
            view: TransientTextureView::TextureView(texture_view),
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(context.options.clear_color),
                store: wgpu::StoreOp::Store,
            },
        });
    }
}
