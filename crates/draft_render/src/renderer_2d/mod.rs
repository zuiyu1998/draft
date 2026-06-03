mod resource;

use std::{collections::HashMap, mem::take, ops::Deref};

use bytemuck::{Pod, Zeroable};
use draft_mesh::{Mesh, MeshVertexBufferLayoutRef};
use draft_utils::AffineExt;
use encase::ShaderType;
use wgpu::TextureFormat;

use nalgebra::{Affine3, Vector4};

use crate::{
    FrameworkError,
    render_phase::{
        BindGroupBinding, BindGroupPhase, BufferBinding, MeshPhase, MeshRenderPhase,
        RenderPhaseContainer, ResourceBinding,
    },
    render_resource::{
        BindGroupLayoutDescriptor, BindGroupLayoutEntries, GpuArrayBuffer,
        binding_types::uniform_buffer_sized,
    },
    render_world::{
        CachePipelineId, GpuFragmentState, GpuPipelineLayoutDescriptor,
        GpuRenderPipelineDescriptor, GpuVertexState, RenderWorld, ResourceId,
    },
};

use draft_graphics::{RenderDevice, ShaderStages};

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

pub trait MaterialPipeline: 'static {
    fn specialize(
        &self,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<GpuRenderPipelineDescriptor, FrameworkError>;

    fn create_render_phase(
        &self,
        command: &RenderPhaseCommand,
        render_device: &RenderDevice,
        render_world: &mut RenderWorld,
    ) -> MeshRenderPhase;
}

pub struct Mesh2dMaterialPipeline {}

impl MaterialPipeline for Mesh2dMaterialPipeline {
    fn specialize(
        &self,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<GpuRenderPipelineDescriptor, FrameworkError> {
        let layout = layout.0.get_layout();

        let view = BindGroupLayoutEntries::new(ShaderStages::all())
            .add_entry(uniform_buffer_sized(true, None))
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

    fn create_render_phase(
        &self,
        command: &RenderPhaseCommand,
        render_device: &RenderDevice,
        render_world: &mut RenderWorld,
    ) -> MeshRenderPhase {
        let limits = render_device.limits();

        let mut gpu_array_buffer = GpuArrayBuffer::<Mesh2dUniform>::new(&limits);

        for uniform in command.mesh_uniforms.iter() {
            gpu_array_buffer.push(*uniform);
        }

        let view_index = gpu_array_buffer.write("Mesh2dUniform", render_world);

        let layout = render_world.get_bind_group_layout("view");

        let bind_group_phase = BindGroupPhase {
            index: 0,
            offsets: vec![0],
            binding: BindGroupBinding::Binding {
                resource_bindings: vec![ResourceBinding::Buffer(BufferBinding {
                    uniform_index: view_index,
                    offset: 0,
                    size: None,
                })],
                bind_group_layout: layout,
            },
        };

        MeshRenderPhase {
            mesh: MeshPhase {
                mesh_id: command.mesh_id,
                instances: 0..command.mesh_uniforms.len() as u32,
            },
            pipeline_id: command.pipeline_id,
            bind_groups: vec![bind_group_phase],
        }
    }
}

pub struct MaterialPipelineContainer {
    pipelines: HashMap<String, Box<dyn MaterialPipeline>>,
}

impl MaterialPipelineContainer {
    pub fn new() -> Self {
        let mut container = Self::empty();

        container.add_pipeline("mesh_2d", Box::new(Mesh2dMaterialPipeline {}));

        container
    }

    pub fn empty() -> Self {
        Self {
            pipelines: HashMap::new(),
        }
    }

    pub fn add_pipeline(&mut self, name: &str, pipeline: Box<dyn MaterialPipeline>) {
        self.pipelines.insert(name.to_string(), pipeline);
    }

    pub fn get_pipeline(&self, name: &str) -> Option<&Box<dyn MaterialPipeline>> {
        self.pipelines.get(name)
    }
}

pub struct Mesh2dTransform {
    pub world_from_local: Affine3<f32>,
}

#[derive(PartialEq, Hash, Clone, Eq)]
pub struct MeshMaterial {
    mesh_id: ResourceId<Mesh>,
}

pub struct RenderPhaseCommand {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
    pub mesh_uniforms: Vec<Mesh2dUniform>,
}

impl RenderPhaseCommand {
    pub fn add_mesh_uniform(&mut self, mesh_uniform: Mesh2dUniform) {
        self.mesh_uniforms.push(mesh_uniform);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct MeshMaterialBuilderKey {
    pub mesh_id: ResourceId<Mesh>,
    pub pipeline_id: CachePipelineId,
}

#[derive(Default)]
pub struct RenderPhaseCommandContainer(HashMap<MeshMaterialBuilderKey, RenderPhaseCommand>);

impl Deref for RenderPhaseCommandContainer {
    type Target = HashMap<MeshMaterialBuilderKey, RenderPhaseCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderPhaseCommandContainer {
    pub fn get_or_create_render_phase_command(
        &mut self,
        mesh_id: ResourceId<Mesh>,
        pipeline_id: CachePipelineId,
    ) -> &mut RenderPhaseCommand {
        let key = MeshMaterialBuilderKey {
            mesh_id,
            pipeline_id,
        };
        self.0.entry(key).or_insert_with(|| RenderPhaseCommand {
            mesh_id,
            pipeline_id,
            mesh_uniforms: vec![],
        })
    }
}

pub struct Renderer2d {
    mesh_material_cache: HashMap<MeshMaterial, CachePipelineId>,
    pub render_phase_command_container: RenderPhaseCommandContainer,
    material_pipeline_container: MaterialPipelineContainer,
}

impl Renderer2d {
    pub fn spawn_render_phase(
        &mut self,
        render_device: &RenderDevice,
        render_world: &mut RenderWorld,
        render_phase_container: &mut RenderPhaseContainer,
    ) {
        let render_phase_commands = take(&mut self.render_phase_command_container);

        let material_pipeline = self
            .material_pipeline_container
            .get_pipeline("mesh_2d")
            .unwrap();

        for render_phase_command in render_phase_commands.values() {
            let render_phase = material_pipeline.create_render_phase(
                render_phase_command,
                render_device,
                render_world,
            );

            render_phase_container.add(CORE_2D, render_phase);
        }
    }

    pub fn draw_mesh(
        &mut self,
        mesh_id: ResourceId<Mesh>,
        pipeline_id: CachePipelineId,
        mesh_transform: Mesh2dTransform,
    ) {
        let render_phase_command = self
            .render_phase_command_container
            .get_or_create_render_phase_command(mesh_id, pipeline_id);

        render_phase_command.add_mesh_uniform(Mesh2dUniform::from_transform(&mesh_transform));
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

        let material_pipeline = self
            .material_pipeline_container
            .get_pipeline("mesh_2d")
            .unwrap();

        let desc = material_pipeline.specialize(&layout)?;

        let id = render_world.create_render_pipeline(desc)?;

        self.mesh_material_cache.insert(mesh_material, id);

        Ok(id)
    }
}

impl Default for Renderer2d {
    fn default() -> Self {
        Self {
            mesh_material_cache: HashMap::default(),
            render_phase_command_container: Default::default(),
            material_pipeline_container: MaterialPipelineContainer::new(),
        }
    }
}
