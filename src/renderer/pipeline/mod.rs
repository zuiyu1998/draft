use std::ops::{Deref, DerefMut};

use draft_render::{
    FrameworkError, GeometryResource, MaterialResource, RenderPhasesContainer, RenderWorld,
    frame_graph::{FrameGraph, TextureView},
};
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

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
        _render_phases_container: &mut RenderPhasesContainer,
    ) -> Result<(), FrameworkError> {
        let geometry_data = world
            .geometry_cache
            .get_or_create(&world.server.device, &self.geometry)?;

        let _vertex_layout = geometry_data.layout.clone();
        let _vertex_buffer = geometry_data.get_vertex_buffer();
        let _index_buffer = geometry_data.get_index_buffer();

        let material_state = self.material.state();
        let Some(_material_state) = material_state.data_ref() else {
            return Err(self.material.clone().into());
        };

        todo!()
    }
}

pub struct PipelineContext<'a> {
    pub batch: &'a Batch,
    pub texture_view: TextureView,
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

pub struct PipelineContainer(FxHashMap<ImmutableString, Pipeline>);

impl Default for PipelineContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineContainer {
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn new() -> Self {
        PipelineContainer::empty()
    }
}

impl Deref for PipelineContainer {
    type Target = FxHashMap<ImmutableString, Pipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PipelineContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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
