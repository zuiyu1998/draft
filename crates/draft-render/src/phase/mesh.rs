use draft_gfx_base::CachedPipelineId;
use fyrox_core::ImmutableString;
use wgpu::BufferUsages;

use crate::{
    IndexRenderBuffer, MaterialEffectData, MaterialResourceHandle, PhaseName, RenderPhase,
    RenderWorld, frame_graph::RenderPassBuilder, render_resource::RenderBuffer,
};

pub struct MeshRenderPhase {
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
    pub pipeline_id: CachedPipelineId,
    pub material_effect_data: Vec<MaterialEffectData>,
}

impl PhaseName for MeshRenderPhase {
    fn name() -> fyrox_core::ImmutableString {
        ImmutableString::new("MeshRenderPhase")
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld) {
        if !world.pipeline_cache.has_render_pipeline(self.pipeline_id) {
            return;
        }

        render_pass_builder.set_render_pipeline(self.pipeline_id);

        for (index, material_effect_data) in self.material_effect_data.iter().enumerate() {
            let mut bind_group_handle_builder = render_pass_builder
                .create_bind_group_handle_builder(
                    None,
                    material_effect_data
                        .bind_group_layout
                        .get_bind_group_layout()
                        .clone(),
                );

            let mut offsets = vec![];

            for (binding, handle) in material_effect_data.handles.iter().enumerate() {
                match handle {
                    MaterialResourceHandle::Texture(material_texture_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_texture_view(binding as u32, &material_texture_handle.texture);
                    }
                    MaterialResourceHandle::Sampler(material_sampler_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_handle(binding as u32, &material_sampler_handle.sampler);
                    }
                    MaterialResourceHandle::PropertyGroup(material_property_group_handle) => {
                        let handle = world.material_buffer_handle_cache.get_or_create(
                            &mut world.buffer_allocator,
                            &mut world.buffer_cache,
                            bind_group_handle_builder.frame_graph_mut(),
                            material_property_group_handle,
                            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        );
                        offsets.push(material_property_group_handle.offset);
                        bind_group_handle_builder =
                            bind_group_handle_builder.add_handle(binding as u32, &handle);
                    }
                    MaterialResourceHandle::Buffer(material_buffer_handle) => {
                        offsets.push(material_buffer_handle.offset);
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_handle(binding as u32, &material_buffer_handle.handle);
                    }
                }
            }

            let bind_group_handle = bind_group_handle_builder.build();

            render_pass_builder.set_bind_group_handle(index as u32, &bind_group_handle, &offsets);
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
