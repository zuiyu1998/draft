use fyrox_core::ImmutableString;

use crate::{
    IndexRenderBuffer, MaterialBindGroupHandle, MaterialResourceHandle, PhaseName, RenderPhase,
    RenderWorld, frame_graph::RenderPassBuilder, gfx_base::CachedPipelineId,
    render_resource::RenderBuffer,
};

pub struct MeshRenderPhase {
    pub pipeline_id: CachedPipelineId,
    pub bind_groups: Vec<MaterialBindGroupHandle>,
    pub vertex_buffer: RenderBuffer,
    pub index_buffer: Option<IndexRenderBuffer>,
}

impl PhaseName for MeshRenderPhase {
    fn name() -> fyrox_core::ImmutableString {
        ImmutableString::new("MeshRenderPhase")
    }
}

impl RenderPhase for MeshRenderPhase {
    fn render(&self, render_pass_builder: &mut RenderPassBuilder, world: &RenderWorld) {
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
