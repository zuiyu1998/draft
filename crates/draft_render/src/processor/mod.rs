mod resource;

use std::collections::HashMap;

use draft_mesh::{Mesh, MeshVertexBufferLayoutRef};

use crate::{
    FrameworkError,
    render_world::{
        GpuFragmentState, GpuRenderPipelineDescriptor, GpuVertexState, RenderWorld, ResourceId,
    },
};

use draft_graphics::{
    BlendState, ColorTargetState, ColorWrites, MultisampleState, PrimitiveState, TextureFormat,
};

pub const CORE_2D: &'static str = "core_2d";

pub use resource::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MeshMateridlKey {
    pub layout: MeshVertexBufferLayoutRef,
}

pub struct Processor {}

impl Processor {
    pub fn create_render_pipeline_descriptor(
        &mut self,
        _layout: MeshVertexBufferLayoutRef,
    ) -> GpuRenderPipelineDescriptor {
        GpuRenderPipelineDescriptor {
            label: "test".to_string(),
            vertex: GpuVertexState {
                shader: SHADER.resource(),
                entry_point: Some("vs_main".into()),
                buffers: vec![],
            },
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // 将此设置为 Fill 以外的任何值都要需要开启 Feature::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // 需要开启 Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // 需要开启 Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            fragment: Some(GpuFragmentState {
                shader: SHADER.resource(),
                entry_point: Some("fs_main".into()),
                targets: vec![Some(ColorTargetState {
                    // 4.
                    format: TextureFormat::Bgra8Unorm,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            zero_initialize_workgroup_memory: false,
        }
    }

    pub fn create_render_pipeline(
        &mut self,
        render_world: &mut RenderWorld,
        mesh_id: ResourceId<Mesh>,
    ) -> Result<(), FrameworkError> {
        let mesh = render_world
            .get_mesh(mesh_id)
            .ok_or(FrameworkError::MeshNotFound)?;

        let layout = render_world.get_mesh_vertex_buffer_layout(&mesh);
        let desc = self.create_render_pipeline_descriptor(layout);

        render_world.get_or_create_render_pipeline(desc)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct ProcessorContianer(HashMap<String, Processor>);

impl ProcessorContianer {
    pub fn get_processor(&mut self, name: &str) -> Option<&Processor> {
        self.0.get(name)
    }
}
