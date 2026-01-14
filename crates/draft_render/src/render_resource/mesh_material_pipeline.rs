use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{
    CachedRenderPipelineId, MaterialEffectInstance, PipelineCache, RenderPipelineDescriptor,
    render_resource::VertexState,
};
use draft_graphics::gfx_base::BindGroupLayout;
use draft_material::{MaterialFragmentState, MaterialResource, MaterialVertexState, PipelineState};

use draft_mesh::{MeshResource, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts};
use fxhash::FxHasher;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MeshKey {
    id: u64,
    layout: MeshVertexBufferLayoutRef,
}

impl MeshKey {
    pub fn create(mesh: &MeshResource, layouts: &mut MeshVertexBufferLayouts) -> Self {
        let id = mesh.key();
        let mesh = mesh.data_ref();
        let layout = mesh.get_mesh_vertex_buffer_layout(layouts);
        Self { id, layout }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexStateKey {
    id: u64,
    hash: u64,
}

impl VertexStateKey {
    pub fn from_vertex_state(vertex_state: &MaterialVertexState) -> Self {
        let id = vertex_state.shader.key();

        let shader = vertex_state.shader.data_ref().clone();

        let mut hasher = FxHasher::default();
        shader.hash(&mut hasher);
        vertex_state.entry_point.hash(&mut hasher);
        vertex_state.shader_defs.hash(&mut hasher);

        let hash = hasher.finish();

        VertexStateKey { id, hash }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FragmentStateKey {
    id: u64,
    hash: u64,
}

impl FragmentStateKey {
    pub fn from_fragment_state(fragment_state: &MaterialFragmentState) -> Self {
        let id = fragment_state.shader.key();

        let shader = fragment_state.shader.data_ref().clone();

        let mut hasher = FxHasher::default();
        shader.hash(&mut hasher);
        fragment_state.entry_point.hash(&mut hasher);
        fragment_state.targets.hash(&mut hasher);
        fragment_state.shader_defs.hash(&mut hasher);

        let hash = hasher.finish();

        FragmentStateKey { id, hash }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterialKey {
    id: u64,
    vertex_state: VertexStateKey,
    fragment_state: Option<FragmentStateKey>,
}

impl MaterialKey {
    pub fn from_material(material: &MaterialResource) -> Self {
        let id = material.key();
        let material = material.data_ref();
        let vertex_state = VertexStateKey::from_vertex_state(&material.pipeline_state.vertex);

        let fragment_state = material
            .pipeline_state
            .fragment
            .as_ref()
            .map(|fragment_state| FragmentStateKey::from_fragment_state(fragment_state));

        MaterialKey {
            id,
            vertex_state,
            fragment_state,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MeshPipelineKey {
    mesh: MeshKey,
    material: MaterialKey,
}

fn get_mesh_pipeline_key(
    mesh: &MeshResource,
    material: &MaterialResource,
    layouts: &mut MeshVertexBufferLayouts,
) -> MeshPipelineKey {
    let mesh = MeshKey::create(&mesh, layouts);
    let material = MaterialKey::from_material(&material);

    MeshPipelineKey { mesh, material }
}

#[derive(Default)]
pub struct MeshMaterialPipeline {
    cache: HashMap<MeshPipelineKey, CachedRenderPipelineId>,
}

impl MeshMaterialPipeline {
    pub fn get(
        &mut self,
        mesh: &MeshResource,
        material: &MaterialResource,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut MeshVertexBufferLayouts,
        material_effect_instance: &MaterialEffectInstance,
    ) -> CachedRenderPipelineId {
        let key = get_mesh_pipeline_key(mesh, material, layouts);

        if let Some(id) = self.cache.get(&key) {
            return *id;
        }

        let layout = mesh.data_ref().get_mesh_vertex_buffer_layout(layouts);

        let pipeline_state = material.data_ref().pipeline_state.clone();

        let id = self.specialize(
            pipeline_state,
            pipeline_cache,
            &layout,
            material_effect_instance,
        );

        self.cache.insert(key, id);

        id
    }

    pub fn specialize(
        &mut self,
        pipeline_state: PipelineState,
        pipeline_cache: &mut PipelineCache,
        layout: &MeshVertexBufferLayoutRef,
        material_effect_instance: &MaterialEffectInstance,
    ) -> CachedRenderPipelineId {
        let buffer = layout.0.layout().clone();

        let layout = material_effect_instance
            .bind_groups
            .iter()
            .map(|bind_group| bind_group.bind_group_layout.deref().clone())
            .collect::<Vec<BindGroupLayout>>();

        pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout,
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: pipeline_state.vertex.shader,
                entry_point: pipeline_state.vertex.entry_point,
                shader_defs: pipeline_state.vertex.shader_defs,
                buffers: vec![buffer],
            },
            fragment: pipeline_state.fragment,
            depth_stencil: pipeline_state.depth_stencil,
            multisample: pipeline_state.multisample,
            primitive: pipeline_state.primitive,
            zero_initialize_workgroup_memory: false,
        })
    }
}
