use draft_render::{
    FrameworkError, GeometryResource, IndexRenderBuffer, MaterialBindGroupHandle, MaterialResource,
    MaterialResourceHandle, MaterialResourceHandleContainer, MeshRenderPhase, PhasesContainer,
    RenderPhaseContainer, RenderWorld, TextureResource,
    frame_graph::{FrameGraph, RenderPassBuilder},
    gfx_base::{CachedPipelineId, RawTextureView},
    render_resource::RenderBuffer,
};

pub struct MeshPhase {
    pub pipeline_id: CachedPipelineId,
    pub bind_groups: Vec<MaterialBindGroupHandle>,
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
}

impl MeshRenderPhase for MeshPhase {
    fn render_mesh(&self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld) {
        if !world.pipeline_cache.has_render_pipeline(self.pipeline_id) {
            return;
        }

        render_pass_builder.set_render_pipeline(self.pipeline_id);

        for (index, bind_group) in self.bind_groups.iter().enumerate() {
            let mut bind_group_handle_builder = render_pass_builder
                .create_bind_group_handle_builder(None, bind_group.bind_group_layout.raw().clone());

            for handle in bind_group.material_resource_handle_container.iter() {
                match handle {
                    MaterialResourceHandle::Texture(material_texture_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder.add_texture_view(
                            material_texture_handle.binding,
                            &material_texture_handle.texture,
                        );
                    }
                    MaterialResourceHandle::Sampler(material_sampler_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder.add_handle(
                            material_sampler_handle.binding,
                            &material_sampler_handle.sampler,
                        );
                    }
                    MaterialResourceHandle::PropertyGroup(_material_property_group_handle) => {
                        todo!()
                    }
                }
            }

            let bind_group_handle = bind_group_handle_builder.build();

            render_pass_builder.set_bind_group_handle(index as u32, &bind_group_handle, &[]);
        }

        let buffer_ref = render_pass_builder.read_material(&self.vertex_buffer);
        let buffer_slice = self.vertex_buffer.slice(0..);
        render_pass_builder.set_vertex_buffer(
            0,
            &buffer_ref,
            buffer_slice.offset,
            buffer_slice.size,
        );

        if let Some(index_buffer) = &self.index_buffer {
            let buffer_ref = render_pass_builder.read_material(&index_buffer.buffer);
            let buffer_slice = index_buffer.buffer.slice(0..);
            render_pass_builder.set_index_buffer(
                &buffer_ref,
                index_buffer.index_format,
                buffer_slice.offset,
                buffer_slice.size,
            );

            render_pass_builder.draw_indexed(0..index_buffer.num_indices, 0, 0..1);
        } else {
            render_pass_builder.draw(0..3, 0..1);
        }
    }
}

pub trait MeshPhaseExtractor {
    fn extra(
        &self,
        world: &mut RenderWorld,
        phases_container: &mut PhasesContainer,
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
        phases_container: &mut PhasesContainer,
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

        let phase = MeshPhase {
            pipeline_id,
            bind_groups,
            vertex_buffer,
            index_buffer,
        };

        let render_phase = RenderPhaseContainer::new(phase);

        phases_container.insert(render_phase.name(), render_phase);

        Ok(())
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
