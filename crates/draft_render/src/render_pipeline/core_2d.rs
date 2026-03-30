use crate::render_pipeline::{Node, RenderPipeline, RenderPipelineContext};

pub const CORE_2D: &str = "core_2d";

pub fn create_core_2d_pipeline() -> RenderPipeline {
    let mut pipeline = RenderPipeline::new();
    pipeline.nodes.push(Box::new(Opaque2DNode));
    pipeline
}

pub struct Opaque2DNode;

impl Node for Opaque2DNode {
    fn run(&self, context: &mut RenderPipelineContext) {
        let window_surface_texture = context.get_window_surface_texture();

        let texture_view =
            window_surface_texture
                .surface
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    ..Default::default()
                });

        // Renders a GREEN screen
        let mut encoder = context
            .render_server
            .device
            .wgpu_device()
            .create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        drop(renderpass);

        context.render_server.queue.0.submit([encoder.finish()]);
    }
}
