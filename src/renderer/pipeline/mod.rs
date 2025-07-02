use draft_render::{
    GeometryResource, MaterialResource, RenderWorld, TextureResource,
    frame_graph::FrameGraph,
    gfx_base::{RawTextureView, VertexBufferLayout},
};

pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
    layouts: Vec<VertexBufferLayout>,
}

impl Batch {
    pub fn new(geometry: GeometryResource, material: MaterialResource) -> Self {
        let geometry_clone = geometry.clone();
        let geometry_state = geometry_clone.state();
        let layouts = vec![
            geometry_state
                .data_ref()
                .unwrap()
                .vertex
                .get_vertex_layout(),
        ];

        Self {
            geometry,
            material,
            layouts,
        }
    }

    pub fn layouts(&self) -> &[VertexBufferLayout] {
        &self.layouts
    }
}

pub struct PipelineContext<'a> {
    pub batch: &'a Batch,
    pub texture_view: RawTextureView,
    pub image: &'a TextureResource,
}

pub trait PipelineNode: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        context: &PipelineContext,
    );
}

pub struct Pipeline {
    nodes: Vec<Box<dyn PipelineNode>>,
}

impl Pipeline {
    pub fn empty() -> Self {
        Pipeline { nodes: vec![] }
    }

    pub fn push_node<T: PipelineNode>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        context: &PipelineContext,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, context);
        }
    }
}
