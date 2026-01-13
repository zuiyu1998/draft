mod processor;

pub use processor::*;

use crate::{
    BatchMeshMaterialContainer, BatchRenderMeshMaterialContainer, BufferAllocator,
    IntoMeshMaterialInstanceData, MaterialEffectCache, MeshCache, MeshMaterialInstanceData,
    MeshMaterialPipeline, PipelineCache, RenderDataBundle, RenderFrame, RenderPipeline,
    RenderPipelineContext, RenderPipelineExt, RenderPipelineManager, RenderServer, RenderWindow,
    RenderWindows, error::FrameworkError,
};
use draft_graphics::{
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
    gfx_base::{GetPipelineContainer, TextureView, TextureViewDescriptor},
};
use draft_material::{MaterialEffectResource, MaterialResource};
use draft_mesh::MeshResource;
use draft_shader::Shader;
use draft_window::SystemWindowManager;
use fyrox_resource::{Resource, manager::ResourceManager};
use tracing::error;

pub struct WorldRenderer {
    render_server: RenderServer,
    pipeline_cache: PipelineCache,
    mesh_material_pipeline: MeshMaterialPipeline,
    render_pipeline_manager: RenderPipelineManager,
    system_window_manager: SystemWindowManager,
    buffer_allocator: BufferAllocator,
    transient_resource_cache: TransientResourceCache,
    mesh_cache: MeshCache,
    material_effect_cache: MaterialEffectCache,
    mesh_material_processor: MeshMaterialProcessor,
    mesh_processor: MeshProcessor,
}

impl RenderPipelineExt for WorldRenderer {
    fn insert_pipeline(&mut self, name: &str, pipeline: RenderPipeline) {
        self.render_pipeline_manager.insert_pipeline(name, pipeline);
    }

    fn pipeline(&self, name: &str) -> Option<&RenderPipeline> {
        self.render_pipeline_manager.pipeline(name)
    }

    fn pipeline_mut(&mut self, name: &str) -> Option<&mut RenderPipeline> {
        self.render_pipeline_manager.pipeline_mut(name)
    }
}

impl WorldRenderer {
    pub fn new(
        render_server: RenderServer,
        system_window_manager: SystemWindowManager,
        resource_manager: &ResourceManager,
    ) -> Self {
        Self {
            material_effect_cache: MaterialEffectCache::new(
                render_server.device.clone(),
                resource_manager,
            ),
            pipeline_cache: PipelineCache::new(render_server.device.clone(), resource_manager),
            buffer_allocator: BufferAllocator::new(render_server.device.clone()),
            render_server,
            mesh_material_pipeline: Default::default(),
            render_pipeline_manager: RenderPipelineManager::default(),
            system_window_manager,
            transient_resource_cache: Default::default(),
            mesh_cache: Default::default(),
            mesh_material_processor: MeshMaterialProcessor::new(),
            mesh_processor: MeshProcessor::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pipeline_cache.update();
        self.mesh_cache.update(dt);
        self.material_effect_cache.update(dt);
        self.buffer_allocator.update(dt);
    }

    fn create_render_world(&mut self) -> RenderWorld<'_> {
        RenderWorld::new(
            &mut self.mesh_material_pipeline,
            &mut self.pipeline_cache,
            &mut self.mesh_processor,
        )
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        self.pipeline_cache.set_shader(shader);
    }

    pub fn set_material_effect(&mut self, material_effect: MaterialEffectResource) {
        self.material_effect_cache
            .set_material_effect(material_effect);
    }

    fn prepare_render_windows(&self) -> Result<RenderWindows, FrameworkError> {
        let mut windows = RenderWindows::default();

        for (index, window) in self.system_window_manager.get_ref().iter().enumerate() {
            if let Some(current_texture) = window.get_current_texture() {
                let current_texture = current_texture?;

                let texure_view = TextureView::new(
                    current_texture.create_view(&TextureViewDescriptor::default()),
                );

                windows.insert(
                    index,
                    RenderWindow {
                        surface_texture: current_texture,
                        surface_texture_view: texure_view,
                    },
                );
            }
        }

        let primary = self.system_window_manager.get_primary();

        windows.set_primary(primary.index() as usize);

        Ok(windows)
    }

    fn clear_render_windows(&self, windows: RenderWindows) {
        for window in windows.into_iter() {
            window.surface_texture.present();
        }
    }

    fn prepare_mesh_materials(
        &mut self,
        mesh_materials: BatchMeshMaterialContainer,
    ) -> BatchRenderMeshMaterialContainer {
        self.mesh_material_processor.process(
            mesh_materials,
            &mut self.pipeline_cache,
            &mut self.material_effect_cache,
        )
    }

    fn prepare_frame<W: World>(&mut self, world: &W) -> Result<RenderFrame, FrameworkError> {
        self.buffer_allocator.unset();

        let windows = self.prepare_render_windows()?;

        let mut render_world = self.create_render_world();

        world.prepare(&mut render_world);

        let render_data_bundle = render_world.prepare_render_data_bundle();

        self.mesh_processor.update_cache(&mut self.mesh_cache);
        self.mesh_cache.allocate_and_free_meshes(
            &mut self.buffer_allocator,
            &self.render_server.device,
            &self.render_server.queue,
            &mut self.mesh_processor.vertex_buffer_layouts,
        );

        let mesh_materials = self.prepare_mesh_materials(render_data_bundle.mesh_materials);

        Ok(RenderFrame {
            windows,
            mesh_materials,
        })
    }

    fn render_frame(&mut self, frame: RenderFrame) {
        self.render_pipeline_manager.update();

        let pipeline_container = self.pipeline_cache.get_pipeline_container();

        let context = RenderPipelineContext {
            pipeline_container: &pipeline_container,
            mesh_allocator: &self.mesh_cache.allocator,
            render_device: &self.render_server.device,
        };
        let mut frame_graph = FrameGraph::default();

        if let Some(pipeline) = self.render_pipeline_manager.pipeline_mut("core_2d") {
            pipeline.run(&mut frame_graph, &frame, &context);
        }

        frame_graph.compile();

        let mut frame_graph_context = FrameGraphContext::new(
            pipeline_container,
            &self.render_server.device,
            &mut self.transient_resource_cache,
        );

        frame_graph.execute(&mut frame_graph_context);

        let command_buffers = frame_graph_context.finish();

        self.render_server.queue.submit(command_buffers);

        self.clear_render_windows(frame.windows);
    }

    pub fn render<W: World>(&mut self, world: &W) {
        match self.prepare_frame(world) {
            Ok(frame) => {
                self.render_frame(frame);
            }
            Err(e) => {
                error!("Render error: {}", e);
            }
        };
    }
}

pub struct RenderWorld<'a> {
    mesh_materials: BatchMeshMaterialContainer,
    mesh_material_pipeline: &'a mut MeshMaterialPipeline,
    pipeline_cache: &'a mut PipelineCache,
    mesh_processor: &'a mut MeshProcessor,
}

impl<'a> RenderWorld<'a> {
    pub fn new(
        mesh_material_pipeline: &'a mut MeshMaterialPipeline,
        pipeline_cache: &'a mut PipelineCache,
        mesh_processor: &'a mut MeshProcessor,
    ) -> Self {
        Self {
            mesh_materials: BatchMeshMaterialContainer::default(),
            mesh_material_pipeline,
            pipeline_cache,
            mesh_processor,
        }
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

        self.mesh_processor.prepare(&mesh);

        self.mesh_materials.push(
            mesh,
            material,
            instance_data,
            &mut self.mesh_processor.vertex_buffer_layouts,
            self.mesh_material_pipeline,
            self.pipeline_cache,
        );
    }

    pub fn prepare_render_data_bundle(self) -> RenderDataBundle {
        RenderDataBundle {
            mesh_materials: self.mesh_materials,
        }
    }
}

pub trait World {
    fn prepare(&self, render_wrold: &mut RenderWorld);
}
