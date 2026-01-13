use crate::{
    BufferAllocator, IntoMeshMaterialInstanceData, MaterialEffectCache, MeshCache,
    MeshMaterialInstanceData, MeshMaterialProcessor, MeshProcessor, PipelineCache,
    RenderDataBundle, RenderServer,
};

use draft_graphics::gfx_base::GetPipelineContainer;
use draft_material::{MaterialEffectResource, MaterialResource};
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use draft_shader::Shader;
use fyrox_resource::{Resource, manager::ResourceManager};

pub struct RenderWorld {
    render_server: RenderServer,
    pub pipeline_cache: PipelineCache,
    mesh_processor: MeshProcessor,
    mesh_material_processor: MeshMaterialProcessor,
    pub mesh_cache: MeshCache,
    pub mesh_vertex_buffer_layouts: MeshVertexBufferLayouts,
    pub buffer_allocator: BufferAllocator,
    material_effect_cache: MaterialEffectCache,
}

impl RenderWorld {
    pub fn new(render_server: &RenderServer, resource_manager: &ResourceManager) -> Self {
        Self {
            material_effect_cache: MaterialEffectCache::new(
                render_server.device.clone(),
                resource_manager,
            ),
            pipeline_cache: PipelineCache::new(render_server.device.clone(), resource_manager),
            buffer_allocator: BufferAllocator::new(render_server.device.clone()),
            mesh_processor: Default::default(),
            mesh_material_processor: MeshMaterialProcessor::new(),
            mesh_cache: Default::default(),
            mesh_vertex_buffer_layouts: Default::default(),
            render_server: render_server.clone(),
        }
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        self.pipeline_cache.set_shader(shader);
    }

    pub fn set_material_effect(&mut self, material_effect: MaterialEffectResource) {
        self.material_effect_cache
            .set_material_effect(material_effect);
    }

    pub fn update(&mut self, dt: f32) {
        self.pipeline_cache.update();
        self.mesh_cache.update(dt);
        self.material_effect_cache.update(dt);
        self.buffer_allocator.update(dt);
    }

    pub fn push(
        &mut self,
        mesh: MeshResource,
        material: MaterialResource,
        instance: impl IntoMeshMaterialInstanceData,
    ) {
        let instance_data = instance.into_mesh_material_instance_data();
        self.push_with_instance_data(mesh, material, instance_data);
    }

    pub fn push_with_instance_data(
        &mut self,
        mesh: MeshResource,
        material: MaterialResource,
        instance_data: MeshMaterialInstanceData,
    ) {
        if !mesh.is_ok() {
            return;
        }

        if !material.is_ok() {
            return;
        }

        let effect_name = material.data_ref().effct_info.effect_name.clone();

        let Some(material_effect_instance) = self
            .material_effect_cache
            .get_material_effect_instance(&effect_name)
        else {
            return;
        };

        self.mesh_processor.process(&mesh);

        self.mesh_material_processor.process(
            &mesh,
            &material,
            &instance_data,
            &mut self.pipeline_cache,
            &mut self.mesh_vertex_buffer_layouts,
            &material_effect_instance,
        );
    }

    pub fn prepare_render_data_bundle(&mut self) -> RenderDataBundle {
        self.mesh_processor.update_cache(
            &mut self.mesh_cache,
            &mut self.buffer_allocator,
            &self.render_server.device,
            &self.render_server.queue,
            &mut self.mesh_vertex_buffer_layouts,
        );

        let _pipeline_container = self.pipeline_cache.get_pipeline_container();

        todo!()
    }
}

pub trait World {
    fn prepare(&self, render_wrold: &mut RenderWorld);
}
