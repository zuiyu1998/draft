use draft_render::{
    RenderServer, RenderWorld, SceneRenderData,
    frame_graph::{ColorAttachmentOwned, FrameGraph, RenderContext, TransientResourceCache},
    gfx_base::{Color, LoadOp, Operations, StoreOp},
};

pub struct WorldRenderer {
    pub world: RenderWorld,
    pub node: MainOpaquePass2dNode,
    pub transient_resource_cache: TransientResourceCache,
}

impl WorldRenderer {
    pub fn new(server: RenderServer) -> Self {
        WorldRenderer {
            world: RenderWorld::new(server, Default::default()),
            node: MainOpaquePass2dNode,
            transient_resource_cache: Default::default(),
        }
    }

    pub fn render(&mut self, scene_render_data: SceneRenderData) {
        let mut frame_graph = FrameGraph::default();

        let mut frame_graph_context = FrameGraphContext {
            frame_graph: &mut frame_graph,
            world: &mut self.world,
            scene_render_data,
        };

        //setup
        self.node.run(&mut frame_graph_context);

        frame_graph.compile();

        let mut render_context = RenderContext::new(
            &self.world.server().device,
            &mut self.transient_resource_cache,
            &self.world.render_storage.material_storage,
        );

        frame_graph.execute(&mut render_context);

        let command_buffers = render_context.finish();

        self.world
            .server()
            .queue
            .wgpu_queue()
            .submit(command_buffers);
    }
}

pub struct MainOpaquePass2dNode;

impl FrameGraphNode for MainOpaquePass2dNode {
    fn run(&mut self, context: &mut FrameGraphContext) {
        let mut pass_builder = context.frame_graph.create_pass_builder("test_node");
        let mut render_pass_builder = pass_builder.create_render_pass_builder("test_pass");

        render_pass_builder.add_raw_color_attachment(ColorAttachmentOwned {
            view: context.scene_render_data.texture_view.clone(),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::BLUE),
                store: StoreOp::Store,
            },
        });

        let material_data = context
            .world
            .get_material_data(&context.scene_render_data.batch.material)
            .unwrap();

        render_pass_builder.set_render_pipeline(material_data.pipeline_id);

        let geometry_data = context
            .world
            .get_geometry_data(&context.scene_render_data.batch.geometry)
            .unwrap();

        let buffer_ref = render_pass_builder.read_material(&geometry_data.vertex_buffer);
        let buffer_slice = geometry_data.vertex_buffer.slice(0..);

        render_pass_builder.set_vertex_buffer(
            0,
            &buffer_ref,
            buffer_slice.offset,
            buffer_slice.size,
        );
        render_pass_builder.draw(0..3, 0..1);
    }
}

pub trait FrameGraphNode {
    fn run(&mut self, context: &mut FrameGraphContext);
}

pub struct FrameGraphContext<'a> {
    pub frame_graph: &'a mut FrameGraph,
    pub world: &'a mut RenderWorld,
    pub scene_render_data: SceneRenderData<'a>,
}
