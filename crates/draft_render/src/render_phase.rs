mod draw_state;

use std::{collections::HashMap, num::NonZeroU64, ops::Range, sync::Arc};

use draft_graphics::{BindGroup, BindGroupLayout, Buffer};
use draft_mesh::Mesh;

use crate::{
    frame_graph::*,
    render_world::{CachePipelineId, RenderWorld, ResourceId, UniformIndex},
};

pub use draw_state::*;

#[derive(Default)]
pub struct RenderPhaseContainer(HashMap<String, Vec<Box<dyn RenderPhase>>>);

impl RenderPhaseContainer {
    pub fn get(&self, name: &str) -> Option<&Vec<Box<dyn RenderPhase>>> {
        self.0.get(name)
    }

    pub fn add(&mut self, name: &str, render_phase: impl RenderPhase + 'static) {
        self.add_boxed(name, Box::new(render_phase));
    }

    pub fn add_boxed(&mut self, name: &str, render_phase: Box<dyn RenderPhase>) {
        self.0
            .entry(name.to_string())
            .or_default()
            .push(render_phase);
    }
}

pub trait RenderPhase: 'static {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, render_world: &RenderWorld);
}

pub enum ResourceBinding {
    Buffer(BufferBinding),
}

pub struct BufferBinding {
    pub uniform_index: UniformIndex,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

pub enum BindGroupBinding {
    BindGroup(BindGroup),
    Binding {
        resource_bindings: Vec<ResourceBinding>,
        bind_group_layout: BindGroupLayout,
    },
}

pub struct BindGroupPhase {
    pub index: u32,
    pub offsets: Vec<u32>,
    pub binding: BindGroupBinding,
}

impl BindGroupPhase {
    pub fn render(&self, builder: &mut TrackedRenderPassBuilder, render_world: &RenderWorld) {
        let bind_group = match &self.binding {
            BindGroupBinding::BindGroup(bg) => TransientBindGroup::BindGroup(bg.clone()),
            BindGroupBinding::Binding {
                resource_bindings,
                bind_group_layout,
            } => {
                let mut desc = TransientBindGroupDescriptor::new(bind_group_layout.clone());

                let mut binding = 0;

                for resource_binding in resource_bindings {
                    match resource_binding {
                        ResourceBinding::Buffer(info) => {
                            let render_data =
                                render_world.get_uniform_render_data(&info.uniform_index);

                            let material = UniformResourceMaterial::new(
                                render_data.buffer.clone(),
                                info.uniform_index.clone(),
                            );

                            let buffer = builder.read_material(&material);

                            desc.add_entry(
                                binding,
                                BindingBuffer {
                                    buffer_ref: buffer,
                                    offset: info.offset,
                                    size: info.size,
                                },
                            );

                            binding += 1;
                        }
                    }
                }

                TransientBindGroup::Desc(desc)
            }
        };

        builder.set_bind_group(self.index, &bind_group, &self.offsets);
    }
}

pub struct MeshPhase {
    pub mesh_id: ResourceId<Mesh>,
    pub instances: Range<u32>,
}

pub struct MeshRenderPhase {
    pub mesh: MeshPhase,
    pub pipeline_id: CachePipelineId,
    pub bind_groups: Vec<BindGroupPhase>,
}

impl MeshPhase {
    pub fn render(&self, builder: &mut TrackedRenderPassBuilder, render_world: &RenderWorld) {
        let buffer = render_world.get_vertex_buffer(self.mesh_id);

        let material =
            MeshResourceMaterial::new(buffer.clone(), format!("mesh {}", self.mesh_id.slot));
        let buffer_sllice = buffer.slice(..);

        let buffer = builder.read_material(&material);

        builder.set_vertex_buffer(
            0,
            &buffer,
            buffer_sllice.offset(),
            buffer_sllice.size().get(),
        );

        if let Some(index_render_data) = render_world.get_index_buffer(self.mesh_id) {
            let material = MeshResourceMaterial::new(
                index_render_data.buffer.clone(),
                format!("index {}", self.mesh_id.slot),
            );
            let buffer_sllice = index_render_data.buffer.slice(..);

            let buffer = builder.read_material(&material);

            builder.set_index_buffer(
                &buffer,
                index_render_data.index_format,
                buffer_sllice.offset(),
                buffer_sllice.size().get(),
            );

            builder.draw_indexed(0..index_render_data.num_indices, 0, self.instances.clone());
        } else {
            builder.draw(0..3, self.instances.clone());
        }
    }
}

impl MeshRenderPhase {
    pub fn new(
        mesh: MeshPhase,
        pipeline_id: CachePipelineId,
        bind_groups: Vec<BindGroupPhase>,
    ) -> Self {
        Self {
            mesh,
            pipeline_id,
            bind_groups,
        }
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, render_world: &RenderWorld) {
        builder.set_render_pipeline(self.pipeline_id);

        self.bind_groups.iter().for_each(|bind_group| {
            bind_group.render(builder, render_world);
        });

        self.mesh.render(builder, render_world);

        builder.reset();
    }
}

pub struct UniformResourceMaterial {
    buffer: Buffer,
    uniform_index: UniformIndex,
}

impl UniformResourceMaterial {
    pub fn new(buffer: Buffer, uniform_index: UniformIndex) -> Self {
        Self {
            buffer,
            uniform_index,
        }
    }
}

impl ResourceMaterial for UniformResourceMaterial {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> ResourceHandle<Self::ResourceType> {
        let buffer = TransientBuffer {
            resource: self.buffer.clone(),
            desc: TransientBufferDescriptor::External,
        };

        let key = format!(
            "uniform {} {}",
            self.uniform_index.key.name, self.uniform_index.index
        );

        frame_graph.import(&key, Arc::new(buffer))
    }
}

pub struct MeshResourceMaterial {
    buffer: Buffer,
    key: String,
}

impl MeshResourceMaterial {
    pub fn new(buffer: Buffer, key: String) -> Self {
        Self { buffer, key }
    }
}

impl ResourceMaterial for MeshResourceMaterial {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> ResourceHandle<Self::ResourceType> {
        let buffer = TransientBuffer {
            resource: self.buffer.clone(),
            desc: TransientBufferDescriptor::External,
        };

        frame_graph.import(&self.key, Arc::new(buffer))
    }
}
