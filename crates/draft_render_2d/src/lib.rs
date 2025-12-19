use std::sync::LazyLock;

use draft_graphics::{
    ColorTargetState, ColorWrites, TextureFormat,
    frame_graph::{
        FrameGraph, PassNodeBuilderExt, TransientRenderPassColorAttachment, TransientTextureView,
    },
    wgpu::{self},
};

use draft_material::{FragmentState, IMaterial, MaterialInfo, PipelineState, VertexState};
use draft_render::{Node, RenderFrameContext, RenderPipeline};
use draft_shader::{Shader, ShaderResource};
use fyrox_core::uuid;
use fyrox_resource::{embedded_data_source, manager::BuiltInResource, untyped::ResourceKind};

pub static MESH_2D: LazyLock<BuiltInResource<Shader>> = LazyLock::new(|| {
    BuiltInResource::new(
        "__MESH_2D__",
        embedded_data_source!("./mesh2d.wgsl"),
        |data| {
            ShaderResource::new_ok(
                uuid!("f5b02124-9601-452a-9368-3fa2a9703ecd"),
                ResourceKind::External,
                Shader::from_wgsl(String::from_utf8(data.to_vec()).unwrap(), ""),
            )
        },
    )
});

pub struct Material2d;

impl IMaterial for Material2d {
    fn name() -> &'static str {
        "2d"
    }

    fn material_info() -> MaterialInfo {
        let mut info = MaterialInfo::default();

        info.pipeline_state = PipelineState {
            vertex: VertexState {
                entry_point: Some("vertex".to_string()),
                shader: MESH_2D.resource(),
                ..Default::default()
            },
            fragment: Some(FragmentState {
                entry_point: Some("fragment".to_string()),
                shader: MESH_2D.resource(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],

                ..Default::default()
            }),
            ..Default::default()
        };

        info
    }

    fn built_in_shaders() -> Vec<&'static BuiltInResource<Shader>> {
        vec![&MESH_2D]
    }
}

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
            if let Some(pipeline) = context
                .pipeline_container
                .get_render_pipeline(batch.id.id())
            {
                render_pass_buidler.set_render_pipeline(pipeline);

                let buffer_ref = render_pass_buidler.read_material(&batch.get_vertex_buffer_meta());
                let slice = batch.vertex_buffer.slice(0..);
                render_pass_buidler.set_vertex_buffer(0, &buffer_ref, slice.offset, slice.size);

                if batch.index_buffer.is_some() {
                    let index_buffer = batch.index_buffer.as_ref().unwrap();
                    let slice = index_buffer.buffer.slice(0..);

                    let buffer_ref =
                        render_pass_buidler.read_material(&batch.get_index_buffer_meta().unwrap());
                    render_pass_buidler.set_index_buffer(
                        &buffer_ref,
                        wgpu::IndexFormat::Uint32,
                        slice.offset,
                        slice.size,
                    );

                    render_pass_buidler.draw_indexed(0..index_buffer.count as u32, 0, 0..1);
                } else {
                    render_pass_buidler.draw(0..3, 0..1);
                }
            }
        }
    }
}
