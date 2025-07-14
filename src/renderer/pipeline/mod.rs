use draft_render::{
    GeometryResource, MaterialResource, RenderWorld, TextureResource, frame_graph::FrameGraph,
    gfx_base::RawTextureView,
};

pub trait MeshPhase {
    fn prepare(&self, world: &mut RenderWorld);
}

pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
}

impl Batch {
    pub fn new(geometry: GeometryResource, material: MaterialResource) -> Self {
        Self { geometry, material }
    }
}

impl MeshPhase for Batch {
    fn prepare(&self, world: &mut RenderWorld) {
        let geometry_state = self.geometry.state();

        if let Some(geometry_state) = geometry_state.data_ref() {
            let vertex_layout = geometry_state.vertex.get_vertex_layout();

            if let Some(desc) = world
                .pipeline_descriptor_cache
                .get_or_create(&[vertex_layout], &self.material)
            {
                let _ = world.material_cache.get_or_create(
                    &self.material,
                    desc,
                    &mut world.pipeline_cache,
                );
            }
        }
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
