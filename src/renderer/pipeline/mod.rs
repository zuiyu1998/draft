use std::ops::{Deref, DerefMut};

use draft_render::{
    BufferAllocator, FrameworkError, GeometryResource, MaterialEffectContext, MaterialResource,
    MeshRenderPhase, PipelineDescriptor, RenderPhasesContainer, RenderWorld,
    frame_graph::{FrameGraph, TextureView},
};
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

pub struct PhaseContext<'a> {
    pub world: &'a mut RenderWorld,
    pub render_phases_container: &'a mut RenderPhasesContainer,
    pub buffer_allocator: &'a mut BufferAllocator,
}

pub trait MeshPhaseExtractor {
    fn extra(&self, context: &mut PhaseContext) -> Result<MeshRenderPhase, FrameworkError>;
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
    fn extra(&self, context: &mut PhaseContext) -> Result<MeshRenderPhase, FrameworkError> {
        let geometry_data = context
            .world
            .geometry_cache
            .get_or_create(&context.world.server.device, &self.geometry)?;

        let vertex_layout = geometry_data.layout.clone();
        let vertex_buffer = geometry_data.get_vertex_buffer();
        let index_buffer = geometry_data.get_index_buffer();

        let material = self.material.state();
        let Some(material) = material.data_ref() else {
            return Err(self.material.clone().into());
        };

        let pipeline_info = material.pipeline_info.state();

        let Some(pipeline_info) = pipeline_info.data_ref() else {
            return Err(material.pipeline_info.clone().into());
        };

        let mut layouts = vec![];
        let mut material_effect_data = vec![];

        for effect in material.effects.iter() {
            let effect_processor = context
                .world
                .material_effect_processor_container
                .get(&effect.effect_name)
                .unwrap();
            layouts.push(effect_processor.to_bind_group_layout_descriptor());

            let mut context = MaterialEffectContext {
                pipeline_cache: &mut context.world.pipeline_cache,
                device: &mut context.world.server.device,
                queue: &mut context.world.server.queue,
                texture_cache: &mut context.world.texture_cache,
            };

            material_effect_data.push(effect_processor.process(effect, &mut context)?);
        }

        let desc = PipelineDescriptor::new(pipeline_info, &layouts, &[vertex_layout]);

        let pipeline_id = context.world.pipeline_cache.get_or_create(&desc);

        Ok(MeshRenderPhase {
            vertex_buffer,
            index_buffer,
            pipeline_id,
            material_effect_data,
        })
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
