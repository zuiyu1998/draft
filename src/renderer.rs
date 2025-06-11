use draft_render::{
    RenderServer,
    frame_graph::{ColorAttachmentOwned, FrameGraph, RenderContext, TransientResourceCache},
    pipeline_storage::PipelineStorage,
    wgpu::{Color, LoadOp, Operations, StoreOp, TextureView},
};

pub struct WorldRenderer {
    pub server: RenderServer,
    pub pipeline_storage: PipelineStorage,
    pub node: MainOpaquePass2dNode,
    pub transient_resource_cache: TransientResourceCache,
}

impl WorldRenderer {
    pub fn new(server: RenderServer) -> Self {
        WorldRenderer {
            server,
            pipeline_storage: Default::default(),
            node: MainOpaquePass2dNode,
            transient_resource_cache: Default::default(),
        }
    }

    pub fn render(&mut self, texture_view: TextureView) {
        let mut frame_graph = FrameGraph::default();

        let mut frame_graph_context = FrameGraphContext {
            frame_graph: &mut frame_graph,
            pipeline_storage: &mut self.pipeline_storage,
            texture_view,
        };

        //setup
        self.node.run(&mut frame_graph_context);

        frame_graph.compile();

        let mut render_context = RenderContext::new(
            &self.server.device,
            &mut self.transient_resource_cache,
            &self.pipeline_storage,
        );

        frame_graph.execute(&mut render_context);

        let command_buffers = render_context.finish();

        self.server.queue.0.submit(command_buffers);
    }
}

pub struct MainOpaquePass2dNode;

impl FrameGraphNode for MainOpaquePass2dNode {
    fn run(&mut self, context: &mut FrameGraphContext) {
        let mut pass_builder = context.frame_graph.create_pass_builder("test_node");
        let mut render_pass_builder = pass_builder.create_render_pass_builder("test_pass");

        render_pass_builder.add_raw_color_attachment(ColorAttachmentOwned {
            view: context.texture_view.clone(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::BLUE),
                store: StoreOp::Store,
            },
        });
    }
}

pub trait FrameGraphNode {
    fn run(&mut self, context: &mut FrameGraphContext);
}

pub struct FrameGraphContext<'a> {
    pub frame_graph: &'a mut FrameGraph,
    pub pipeline_storage: &'a mut PipelineStorage,
    pub texture_view: TextureView,
}
