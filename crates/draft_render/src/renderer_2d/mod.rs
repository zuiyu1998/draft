mod resource;

use std::collections::HashMap;

use draft_mesh::{Mesh, MeshVertexBufferLayoutRef};
use wgpu::TextureFormat;

use crate::{
    FrameworkError,
    render_phase::RenderPhase,
    render_world::{
        CachePipelineId, GpuFragmentState, GpuRenderPipelineDescriptor, GpuVertexState,
        RenderWorld, ResourceId,
    },
};

pub use resource::*;
pub const CORE_2D: &str = "core_2d";

#[derive(PartialEq, Hash, Clone, Eq)]
pub struct MeshMaterial {
    mesh_id: ResourceId<Mesh>,
}

pub struct Renderer2d {
    mesh_material_cache: HashMap<MeshMaterial, CachePipelineId>,
    pub phases: Vec<RenderPhase>,
}

impl Renderer2d {
    pub fn unset(&mut self) {
        self.phases.clear();
    }

    pub fn add_render_phase(&mut self, mesh_id: ResourceId<Mesh>, pipeline_id: CachePipelineId) {
        self.phases.push(RenderPhase {
            mesh_id,
            pipeline_id,
        });
    }

    pub fn create_render_pipeline(
        &mut self,
        render_world: &mut RenderWorld,
        mesh_id: ResourceId<Mesh>,
    ) -> Result<CachePipelineId, FrameworkError> {
        let mesh_material = MeshMaterial { mesh_id };
        if let Some(id) = self.mesh_material_cache.get(&mesh_material) {
            return Ok(*id);
        }

        let mesh = render_world
            .get_mesh(mesh_id)
            .ok_or_else(|| FrameworkError::MeshNotFound)?;

        let layout = render_world.get_mesh_vertex_buffer_layout(&mesh);

        let desc = self.specialize(&layout)?;

        let id = render_world.create_render_pipeline(desc)?;

        self.mesh_material_cache.insert(mesh_material, id);

        Ok(id)
    }

    pub fn specialize(
        &mut self,
        _layout: &MeshVertexBufferLayoutRef,
    ) -> Result<GpuRenderPipelineDescriptor, FrameworkError> {
        Ok(GpuRenderPipelineDescriptor {
            label: "Render Pipeline".into(),
            vertex: GpuVertexState {
                shader: SHADER.resource(),
                entry_point: Some("vs_main".into()),
                buffers: vec![],
            },
            fragment: Some(GpuFragmentState {
                shader: SHADER.resource(),
                entry_point: Some("fs_main".into()),
                targets: vec![Some(wgpu::ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            zero_initialize_workgroup_memory: false,
        })
    }
}

impl Default for Renderer2d {
    fn default() -> Self {
        Self {
            mesh_material_cache: HashMap::default(),
            phases: Vec::new(),
        }
    }
}
