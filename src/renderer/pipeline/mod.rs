use draft_render::{
    FrameworkError, GeometryResource, MaterialBindGroupHandle, MaterialResource,
    MaterialResourceHandleContainer, MeshRenderPhase, RenderPhasesContainer, RenderWorld,
    frame_graph::FrameGraph, gfx_base::RawTextureView,
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

        let specializer_state = material_state.specializer().state();

        let Some(specializer_state) = specializer_state.data_ref() else {
            return Err(material_state.specializer().clone().into());
        };

        let mut desc = specializer_state.desc.clone();
        {
            let render_pipeline_desc = desc.render_pipeline_descriptor().unwrap();

            render_pipeline_desc.vertex.buffers.push(vertex_layout);
        }

        let pipeline_id = world.pipeline_cache.get_or_create(&desc);

        let desc = desc.render_pipeline_descriptor().unwrap();

        let bind_group_layout_descs = desc.layout.get_bind_group_layout_descs();

        let mut bind_group_layouts = vec![];

        for bind_group_layout_desc in bind_group_layout_descs.iter() {
            let bind_group_layout = world
                .pipeline_cache
                .get_or_create_bind_group_layout(bind_group_layout_desc)?
                .clone();

            bind_group_layouts.push(bind_group_layout);
        }

        let name_containers = desc.layout.get_bind_group_layout_names();

        let mut bind_groups = vec![];

        for (index, bind_group_layout) in bind_group_layouts.into_iter().enumerate() {
            let handle_container = MaterialResourceHandleContainer::extra(
                &name_containers[index],
                material_state.resource_bindings(),
                world,
            )?;

            bind_groups.push(MaterialBindGroupHandle {
                bind_group_layout,
                material_resource_handle_container: handle_container,
            });
        }

        let phase = MeshRenderPhase {
            pipeline_id,
            bind_groups,
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
