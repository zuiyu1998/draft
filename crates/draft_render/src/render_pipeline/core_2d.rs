use draft_graphics::{
    frame_graph::{
        FrameGraph, PassNodeBuilderExt, TransientRenderPassColorAttachment, TransientTextureView,
    },
    wgpu,
};

use crate::{Node, RenderFrameContext, RenderPipeline};

pub fn create_core_2d_render_pipiline() -> RenderPipeline {
    let mut pipeline = RenderPipeline::default();

    pipeline.push_node(UpscalingNode);

    pipeline
}

pub struct UpscalingNode;

impl Node for UpscalingNode {
    fn run(&self, frame_graph: &mut FrameGraph, context: &RenderFrameContext) {
        let mut pass_node_builder = frame_graph.create_pass_buidlder("upscaling_node");
        let mut render_pass_buidler = pass_node_builder.create_render_pass_builder("upscaling");

        let texture_view = context.frame.windows.primary().surface_texture_view.clone();

        render_pass_buidler.add_color_attachment(TransientRenderPassColorAttachment {
            view: TransientTextureView::Owned(texture_view),
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        });

        for batch in context.frame.batchs.iter() {
            render_pass_buidler.set_render_pipeline(batch.id.id());

            let buffer_ref = render_pass_buidler.read_material(&batch.get_vertex_buffer_meta());
            let slice = batch.vertex_buffer.slice(0..);
            render_pass_buidler.set_vertex_buffer(0, &buffer_ref, slice.offset, slice.size);

            if batch.index_buffer.is_some() {
                let buffer_ref =
                    render_pass_buidler.read_material(&batch.get_index_buffer_meta().unwrap());
                let slice = batch.vertex_buffer.slice(0..);
                render_pass_buidler.set_index_buffer(
                    &buffer_ref,
                    wgpu::IndexFormat::Uint32,
                    slice.offset,
                    slice.size,
                );
            } else {
            }
        }
    }
}
