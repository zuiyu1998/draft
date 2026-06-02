mod resource;

use std::{collections::HashMap, mem::take, ops::Deref};

use bytemuck::{Pod, Zeroable, cast_slice};
use draft_mesh::{Mesh, MeshVertexBufferLayoutRef};
use draft_utils::AffineExt;
use encase::ShaderType;
use wgpu::{BufferUsages, TextureFormat};

use nalgebra::{Affine3, Vector4};

use crate::{
    FrameworkError,
    render_phase::{MeshRenderPhase, RenderPhaseContainer},
    render_resource::{
        BindGroupLayoutDescriptor, BindGroupLayoutEntries, binding_types::uniform_buffer,
    },
    render_world::{
        CachePipelineId, GpuFragmentState, GpuPipelineLayoutDescriptor,
        GpuRenderPipelineDescriptor, GpuVertexState, RenderWorld, ResourceId,
    },
};

use draft_graphics::ShaderStages;

pub use resource::*;
pub const CORE_2D: &str = "core_2d";

#[repr(C)]
#[derive(Debug, Pod, Zeroable, Clone, Copy, ShaderType)]
pub struct Mesh2dUniform {
    // Affine 4x3 matrix transposed to 3x4
    pub world_from_local: [Vector4<f32>; 3],
    // 3x3 matrix packed in mat2x4 and f32 as:
    //   [0].xyz, [1].x,
    //   [1].yz, [2].xy
    //   [2].z
    pub local_from_world_transpose_a: [Vector4<f32>; 2],
    pub local_from_world_transpose_b: f32,
}

impl Mesh2dUniform {
    pub fn from_transform(mesh_transform: &Mesh2dTransform) -> Self {
        let (local_from_world_transpose_a, local_from_world_transpose_b) =
            mesh_transform.world_from_local.inverse_transpose_3x3();

        Self {
            world_from_local: mesh_transform.world_from_local.to_transpose(),
            local_from_world_transpose_a,
            local_from_world_transpose_b,
        }
    }
}

pub struct Mesh2dTransform {
    pub world_from_local: Affine3<f32>,
}

#[derive(PartialEq, Hash, Clone, Eq)]
pub struct MeshMaterial {
    mesh_id: ResourceId<Mesh>,
}

pub struct RenderPhaseBuilder {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
    pub mesh_uniforms: Vec<Mesh2dUniform>,
}

impl RenderPhaseBuilder {
    pub fn add_mesh_uniform(&mut self, mesh_uniform: Mesh2dUniform) {
        self.mesh_uniforms.push(mesh_uniform);
    }
}

impl RenderPhaseBuilder {
    pub fn build(&self, render_world: &mut RenderWorld) -> MeshRenderPhase {
        let bytes = cast_slice(&self.mesh_uniforms);

        let _view_index = render_world.upload(
            "Mesh2dUniform",
            bytes,
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        );

        MeshRenderPhase {
            mesh_id: self.mesh_id,
            pipeline_id: self.pipeline_id,
            bind_groups: vec![],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct MeshMaterialBuilderKey {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
}

#[derive(Default)]
pub struct PhaseBuilders(HashMap<MeshMaterialBuilderKey, RenderPhaseBuilder>);

impl Deref for PhaseBuilders {
    type Target = HashMap<MeshMaterialBuilderKey, RenderPhaseBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PhaseBuilders {
    pub fn get_or_create_render_phase_builder(
        &mut self,
        mesh_id: ResourceId<Mesh>,
        pipeline_id: CachePipelineId,
    ) -> &mut RenderPhaseBuilder {
        let key = MeshMaterialBuilderKey {
            mesh_id,
            pipeline_id,
        };
        self.0.entry(key).or_insert_with(|| RenderPhaseBuilder {
            mesh_id,
            pipeline_id,
            mesh_uniforms: vec![],
        })
    }
}

pub struct Renderer2d {
    mesh_material_cache: HashMap<MeshMaterial, CachePipelineId>,
    pub phase_builders: PhaseBuilders,
}

impl Renderer2d {
    pub fn spawn_render_phase(
        &mut self,
        render_phase_container: &mut RenderPhaseContainer,
        render_world: &mut RenderWorld,
    ) {
        let phase_builders = take(&mut self.phase_builders);

        for phase_builder in phase_builders.values() {
            let render_phase = phase_builder.build(render_world);
            render_phase_container.add(CORE_2D, render_phase);
        }
    }

    pub fn draw_mesh(
        &mut self,
        mesh_id: ResourceId<Mesh>,
        pipeline_id: CachePipelineId,
        mesh_transform: Mesh2dTransform,
    ) {
        let phase_builder = self
            .phase_builders
            .get_or_create_render_phase_builder(mesh_id, pipeline_id);

        phase_builder.add_mesh_uniform(Mesh2dUniform::from_transform(&mesh_transform));
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
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<GpuRenderPipelineDescriptor, FrameworkError> {
        let layout = layout.0.get_layout();

        let view = BindGroupLayoutEntries::new(ShaderStages::all())
            .add_entry(uniform_buffer::<Mesh2dUniform>(false))
            .build();

        let view_desc = BindGroupLayoutDescriptor {
            label: "view".into(),
            entries: view,
        };

        let mut pipeline_layout_desc = GpuPipelineLayoutDescriptor::default();

        pipeline_layout_desc.add_bind_group_layout(0, view_desc);

        Ok(GpuRenderPipelineDescriptor {
            label: "Render Pipeline".into(),
            layout: pipeline_layout_desc,
            vertex: GpuVertexState {
                shader: SHADER.resource(),
                entry_point: Some("vs_main".into()),
                buffers: vec![layout],
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
            phase_builders: Default::default(),
        }
    }
}
