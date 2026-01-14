mod resource;

pub use resource::*;

use draft_graphics::{
    ColorTargetState, ColorWrites, TextureFormat,
    frame_graph::{FrameGraph, TransientRenderPassColorAttachment, TransientTextureView},
    wgpu::{self},
};

use draft_material::{
    IMaterial, MaterialEffect, MaterialFragmentState, MaterialInfo, MaterialVertexState,
    PipelineState,
};
use draft_render::{
    Node, RenderFrame, RenderPhase, RenderPhaseContext, RenderPipeline, RenderPipelineContext,
    TrackedRenderPassBuilder,
};
use draft_shader::Shader;
use fyrox_resource::manager::BuiltInResource;

pub struct Material2d;

impl IMaterial for Material2d {
    fn name() -> &'static str {
        "2d"
    }

    fn info() -> MaterialInfo {
        let mut info = MaterialInfo::default();
        info.effct_info.effect_name = "material_effect_2d".to_string();

        info.pipeline_state = PipelineState {
            vertex: MaterialVertexState {
                entry_point: Some("vertex".to_string()),
                shader: MESH_2D.resource(),
                ..Default::default()
            },
            fragment: Some(MaterialFragmentState {
                entry_point: Some("fragment".to_string()),
                shader: MESH_2D.resource(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
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

    fn built_in_material_effects() -> Vec<&'static BuiltInResource<MaterialEffect>> {
        vec![&MATERIAL_EFFECT_2D]
    }
}

pub fn create_core_2d_render_pipiline() -> RenderPipeline {
    let mut pipeline = RenderPipeline::default();

    pipeline.push_node(UpscalingNode);

    pipeline
}

pub struct UpscalingNode;

impl Node for UpscalingNode {
    fn run(
        &self,
        frame_graph: &mut FrameGraph,
        render_frame: &RenderFrame,
        context: &RenderPipelineContext,
    ) {
        let mut pass_node_builder = frame_graph.create_pass_buidlder("upscaling_node");
        let mut render_pass_buidler = pass_node_builder.create_render_pass_builder("upscaling");

        let texture_view = render_frame.windows.primary().surface_texture_view.clone();

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

        let mut tracked = TrackedRenderPassBuilder::new(render_pass_buidler, context.render_device);

        let render_phase_context = RenderPhaseContext {
            pipeline_container: context.pipeline_container,
            mesh_allocator: context.mesh_allocator,
            buffer_allocator: context.buffer_allocator
        };

        for mesh_materials in render_frame.render_data_bundle.mesh_materials.values() {
            for mesh_material in mesh_materials.iter() {
                if let Err(e) = mesh_material.render(&mut tracked, &render_phase_context) {
                    tracing::error!("Mesh material render error: {}", e);
                }
            }
        }
    }
}
