use std::mem::take;

use crate::{
    BatchMeshMaterialContainer, BatchRenderMeshMaterial, BatchRenderMeshMaterialContainer,
    BufferAllocator, MeshAllocator, MeshAllocatorSettings, MeshCache, MeshInstanceData,
    MeshMaterialPipeline, PipelineCache, RenderFrame, RenderMeshInfo, RenderPipeline,
    RenderPipelineContext, RenderPipelineExt, RenderPipelineManager, RenderServer, RenderWindow,
    RenderWindows, error::FrameworkError,
};
use draft_graphics::{
    frame_graph::{FrameGraph, FrameGraphContext, TransientResourceCache},
    gfx_base::{GetPipelineContainer, TextureView, TextureViewDescriptor},
};
use draft_material::MaterialResource;
use draft_mesh::{MeshResource, MeshVertexBufferLayouts};
use draft_shader::Shader;
use draft_window::SystemWindowManager;
use fyrox_resource::{Resource, manager::ResourceManager};
use tracing::error;

pub struct WorldRenderer {
    render_server: RenderServer,
    pipeline_cache: PipelineCache,
    mesh_material_pipeline: MeshMaterialPipeline,
    mesh_materials: BatchMeshMaterialContainer,
    render_pipeline_manager: RenderPipelineManager,
    layouts: MeshVertexBufferLayouts,
    system_window_manager: SystemWindowManager,
    buffer_allocator: BufferAllocator,
    transient_resource_cache: TransientResourceCache,
    mesh_allocator_settings: MeshAllocatorSettings,
    mesh_allocator: MeshAllocator,
    mesh_cache: MeshCache,
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
            pipeline_cache: PipelineCache::new(render_server.device.clone(), resource_manager),
            buffer_allocator: BufferAllocator::new(render_server.device.clone()),
            render_server,
            mesh_material_pipeline: Default::default(),
            render_pipeline_manager: RenderPipelineManager::default(),
            layouts: Default::default(),
            system_window_manager,
            transient_resource_cache: Default::default(),
            mesh_allocator: MeshAllocator::new(),
            mesh_allocator_settings: Default::default(),
            mesh_cache: Default::default(),
            mesh_materials: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pipeline_cache.update();
        self.mesh_cache.update(dt);
    }

    pub fn set_shader(&mut self, shader: Resource<Shader>) {
        self.pipeline_cache.set_shader(shader);
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

    fn prepare_mesh_materials(&mut self) -> BatchRenderMeshMaterialContainer {
        let mesh_materials = take(&mut self.mesh_materials);

        let mut container = BatchRenderMeshMaterialContainer::default();

        for (key, batch_mesh_materials) in mesh_materials.iter() {
            if let Some(_) = self.pipeline_cache.get_pipeline(key.pipeline_id.id()) {
                let batch_range = 0..(batch_mesh_materials.len() as u32);
                let mut batchs = vec![];

                for batch in batch_mesh_materials {
                    batchs.push(BatchRenderMeshMaterial {
                        pipeline_id: key.pipeline_id.id(),
                        mesh_info: RenderMeshInfo::from_mesh(&batch.mesh),
                        material: batch.material.clone(),
                        batch_range: batch_range.clone(),
                    });
                }

                container.insert(key.clone(), batchs);
            }
        }

        container
    }

    fn prepare_frame<W: World>(&mut self, world: &W) -> Result<RenderFrame, FrameworkError> {
        self.buffer_allocator.unset();

        let windows = self.prepare_render_windows()?;

        let mut context = RenderContext {
            mesh_cache: &mut self.mesh_cache,
            mesh_materials: &mut self.mesh_materials,
            layouts: &mut self.layouts,
            mesh_material_pipeline: &mut self.mesh_material_pipeline,
            pipeline_cache: &mut self.pipeline_cache,
        };

        world.prepare(&mut context)?;

        self.mesh_cache.allocate_and_free_meshes(
            &self.mesh_allocator_settings,
            &mut self.layouts,
            &mut self.buffer_allocator,
            &self.render_server.device,
            &self.render_server.queue,
            &mut self.mesh_allocator,
        );

        let mesh_materials = self.prepare_mesh_materials();

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
            mesh_allocator: &self.mesh_allocator,
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

pub struct RenderContext<'a> {
    mesh_materials: &'a mut BatchMeshMaterialContainer,
    mesh_cache: &'a mut MeshCache,
    layouts: &'a mut MeshVertexBufferLayouts,
    mesh_material_pipeline: &'a mut MeshMaterialPipeline,
    pipeline_cache: &'a mut PipelineCache,
}

impl RenderContext<'_> {
    pub fn push(
        &mut self,
        mesh: MeshResource,
        material: MaterialResource,
        instance: MeshInstanceData,
    ) -> Result<(), FrameworkError> {
        self.mesh_cache.insert_mesh(&mesh);

        self.mesh_materials.push(
            mesh,
            material,
            instance,
            self.layouts,
            self.mesh_material_pipeline,
            self.pipeline_cache,
        )?;

        Ok(())
    }
}

pub trait World {
    fn prepare(&self, context: &mut RenderContext) -> Result<(), FrameworkError>;
}
