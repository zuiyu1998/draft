use fyrox_core::ImmutableString;

use crate::{
    IndexRenderBuffer, MaterialBindGroupHandle, MaterialResourceHandle, PhaseName, RenderPhase,
    RenderWorld, frame_graph::RenderPassBuilder, render_resource::RenderBuffer,
};
use draft_gfx_base::CachedPipelineId;

pub struct MeshRenderPhase {
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
    pub pipeline_id: CachedPipelineId,
    pub material_bind_group_handles: Vec<MaterialBindGroupHandle>,
}

impl PhaseName for MeshRenderPhase {
    fn name() -> ImmutableString {
        ImmutableString::new("MeshRenderPhase")
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &mut RenderWorld) {
        if !world.pipeline_cache.has_render_pipeline(self.pipeline_id) {
            return;
        }

        render_pass_builder.set_render_pipeline(self.pipeline_id);

        for (index, material_bind_group_handle) in
            self.material_bind_group_handles.iter().enumerate()
        {
            let mut bind_group_handle_builder = render_pass_builder
                .create_bind_group_handle_builder(
                    None,
                    material_bind_group_handle
                        .bind_group_layout
                        .get_gpu_bind_group_layout()
                        .clone(),
                );

            let mut offsets = vec![];

            for (binding, handle) in material_bind_group_handle
                .material_resource_handles
                .iter()
                .enumerate()
            {
                match handle {
                    MaterialResourceHandle::Texture(material_texture_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_texture_view(binding as u32, material_texture_handle);
                    }
                    MaterialResourceHandle::Sampler(material_sampler_handle) => {
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_handle(binding as u32, &material_sampler_handle.sampler);
                    }
                    MaterialResourceHandle::Buffer(material_buffer_handle) => {
                        offsets.push(material_buffer_handle.offset);
                        bind_group_handle_builder = bind_group_handle_builder
                            .add_buffer(binding as u32, material_buffer_handle);
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
