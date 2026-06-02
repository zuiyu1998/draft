mod draw_state;

use std::{collections::HashMap, sync::Arc};

use draft_graphics::{BindGroupLayout, Buffer};
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

pub struct BindGroupIndex {
    pub uniform_index: UniformIndex,
    pub bind_group_layout: BindGroupLayout,
}

pub struct MeshRenderPhase {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
    pub bind_groups: Vec<BindGroupIndex>,
}

impl MeshRenderPhase {
    pub fn new(
        mesh_id: ResourceId<Mesh>,
        pipeline_id: CachePipelineId,
        bind_groups: Vec<BindGroupIndex>,
    ) -> Self {
        Self {
            mesh_id,
            pipeline_id,
            bind_groups,
        }
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, render_world: &RenderWorld) {
        builder.set_render_pipeline(self.pipeline_id);

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

            builder.draw_indexed(0..index_render_data.num_indices, 0, 0..1);
        } else {
            builder.draw(0..3, 0..1);
        }

        builder.reset();
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
