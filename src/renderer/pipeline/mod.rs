use std::ops::{Deref, DerefMut};

use draft_render::{
    FrameContext, FrameworkError, FrameworkErrorKind, GeometryResource, MaterialEffectContext,
    MaterialResource, MeshRenderPhase, MeshRenderPhaseExtractor, PhaseContext, PipelineDescriptor,
    RenderWorld, ViewRenderPhasesContainers,
    frame_graph::{FrameGraph, TextureView},
};
use fxhash::FxHashMap;
use fyrox_core::ImmutableString;

use crate::renderer::ObserversCollection;

#[derive(Clone)]
pub struct Batch {
    pub geometry: GeometryResource,
    pub material: MaterialResource,
}

impl Batch {
    pub fn new(geometry: GeometryResource, material: MaterialResource) -> Self {
        Self { geometry, material }
    }
}

impl MeshRenderPhaseExtractor for Batch {
    fn extra(&self, context: PhaseContext) -> Result<MeshRenderPhase, FrameworkError> {
        let PhaseContext {
            render_world,
            frame_context,
            camera_offset,
            camera_size,
        } = context;

        let geometry_data = render_world
            .geometry_cache
            .get_or_create(&render_world.server.device, &self.geometry)?;

        let vertex_layout = geometry_data.layout.clone();
        let vertex_buffer = geometry_data.get_vertex_buffer();
        let index_buffer = geometry_data.get_index_buffer();

        let material = self.material.state();
        let Some(material) = material.data_ref() else {
            return Err(self.material.clone().into());
        };

        let mut layouts = vec![];
        let mut material_bind_group_handles = vec![];

        for effect_info in material.effect_infos().iter() {
            let effect = render_world
                .material_effect_container
                .get(&effect_info.effect_name)
                .ok_or(FrameworkErrorKind::MaterialEffectNotFound(
                    effect_info.effect_name.to_string(),
                ))?;

            layouts.push(effect.get_bind_group_layout_descriptor());

            let context = MaterialEffectContext {
                resource_bindings: &material.resource_bindings,
                frame_context,
                camera_offset,
                camera_size,
                world: render_world,
            };

            material_bind_group_handles.push(effect.process(context)?);
        }

        let desc = PipelineDescriptor::new(&material.pipeline_info, &layouts, &[vertex_layout]);

        let pipeline_id = render_world.pipeline_cache.get_or_create(&desc);

        let mesh_phase = MeshRenderPhase {
            vertex_buffer,
            index_buffer,
            pipeline_id,
            material_bind_group_handles,
        };

        Ok(mesh_phase)
    }
}

pub struct PipelineContext {
    pub batch: Batch,
    pub texture_view: TextureView,
    pub observers_collection: ObserversCollection,
}

pub trait PipelineNode: 'static {
    fn run(
        &mut self,
        frame_graph: &mut FrameGraph,
        world: &mut RenderWorld,
        frame_graph_context: &FrameGraphContext,
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

pub struct FrameGraphContext<'a> {
    pub context: &'a PipelineContext,
    pub frame_context: &'a FrameContext,
    pub camera: Option<usize>,
    pub view_render_phases_containers: &'a ViewRenderPhasesContainers,
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
        frame_graph_context: &FrameGraphContext,
    ) {
        for node in self.nodes.iter_mut() {
            node.run(frame_graph, world, frame_graph_context);
        }
    }
}
