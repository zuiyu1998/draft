use draft_render::{
    FrameworkError, GeometryResource, MaterialResource, MeshRenderPhase, RenderPhasesContainer,
    RenderWorld, frame_graph::FrameGraph, gfx_base::RawTextureView,
};

pub trait MeshPhaseExtractor {
    fn extra(
        &self,
        world: &mut RenderWorld,
        render_phases_container: &mut RenderPhasesContainer,
    ) -> Result<(), FrameworkError>;
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
    fn extra(
        &self,
        world: &mut RenderWorld,
        render_phases_container: &mut RenderPhasesContainer,
    ) -> Result<(), FrameworkError> {
        let geometry_data = world
            .geometry_cache
            .get_or_create(&world.server.device, &self.geometry)?;

        let vertex_layout = geometry_data.layout.clone();
        let vertex_buffer = geometry_data.vertex_buffer.clone();
        let index_buffer = geometry_data.index_buffer.clone();

        let material_state = self.material.state();
        let Some(material_state) = material_state.data_ref() else {
            return Err(self.material.clone().into());
        };

        let pipeline_descriptor = material_state.pipeline_descriptor().state();

        let Some(pipeline_descriptor) = pipeline_descriptor.data_ref() else {
            drop(pipeline_descriptor);
            return Err(material_state.pipeline_descriptor().clone().into());
        };

        let mut desc = pipeline_descriptor.clone();
        {
            let render_pipeline_desc = desc.render_pipeline_descriptor_mut().unwrap();

            render_pipeline_desc.vertex.buffers.push(vertex_layout);
        }

        let desc = desc.render_pipeline_descriptor().unwrap();

        let material_render_data = material_state.resource_bindings().extra(desc, world)?;

        let phase = MeshRenderPhase {
            material_render_data,
            vertex_buffer,
            index_buffer,
        };

        render_phases_container.push(phase);

        Ok(())
    }
}

pub struct PipelineContext<'a> {
    pub batch: &'a Batch,
    pub texture_view: RawTextureView,
}

pub trait PipelineNode: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        context: &PipelineContext,
        render_phases_container: &RenderPhasesContainer,
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
        render_phases_container: &RenderPhasesContainer,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, context, render_phases_container);
        }
    }
}
