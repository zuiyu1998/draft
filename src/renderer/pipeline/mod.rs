use draft_render::{
    GeometryResource, MaterialResource, PhasesContainer, PipelineDescriptor,
    RenderPipelineDescriptor, RenderWorld, TextureResource, frame_graph::FrameGraph,
    gfx_base::RawTextureView,
};

pub trait MeshPhaseExtractor {
    fn extra(&self, world: &mut RenderWorld, phases_container: &mut PhasesContainer);
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

impl MeshPhaseExtractor for Batch {
    fn extra(&self, world: &mut RenderWorld, _phases_container: &mut PhasesContainer) {
        let Some(_geometry_data) = world
            .geometry_cache
            .get_or_insert(&world.server.device, &self.geometry)
        else {
            return;
        };

        let geometry_state = self.geometry.state();

        let Some(geometry_state) = geometry_state.data_ref() else {
            return;
        };
        let vertex_layout = geometry_state.vertex.get_vertex_layout();

        let material_state = self.material.state();
        let Some(material_state) = material_state.data_ref() else {
            return;
        };

        let specializer_state = material_state.specializer().state();

        let Some(specializer_state) = specializer_state.data_ref() else {
            return;
        };

        let mut desc = RenderPipelineDescriptor::default();
        desc.vertex.buffers.push(vertex_layout);

        let mut desc = PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc));
        specializer_state.specialize(&mut desc);

        let _pipeline_id = world.pipeline_cache.get_or_create(&desc);
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
        phases_container: &PhasesContainer,
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
        phases_container: &PhasesContainer,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, context, phases_container);
        }
    }
}
