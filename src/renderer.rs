use draft_render::{
    RenderServer,
    frame_graph::{FrameGraph, RenderContext, TransientResourceCache},
    pipeline_storage::PipelineStorage,
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

    pub fn render(&mut self) {
        let mut frame_graph = FrameGraph::default();

        let mut frame_graph_context = FrameGraphContext {
            frame_graph: &mut frame_graph,
            pipeline_storage: &mut self.pipeline_storage,
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
    fn run(&mut self, _context: &mut FrameGraphContext) {}
}

pub trait FrameGraphNode {
    fn run(&mut self, context: &mut FrameGraphContext);
}

pub struct FrameGraphContext<'a> {
    pub frame_graph: &'a mut FrameGraph,
    pub pipeline_storage: &'a mut PipelineStorage,
}
