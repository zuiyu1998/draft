mod mesh_material;
mod window;

pub use mesh_material::*;
pub use window::*;

use draft_graphics::gfx_base::{CachedPipelineId, RenderDevice, RenderQueue};
use draft_material::MaterialResource;

use draft_mesh::{MeshResource, MeshVertexBufferLayouts};

use crate::{
    BufferAllocator, MeshAllocator, MeshAllocatorSettings, MeshCache, PipelineCache, RenderPhase,
    RenderPhaseContext, SpecializedMeshPipeline, TrackedRenderPassBuilder, error::FrameworkError,
};

pub struct Frame {
    pub windows: RenderWindows,
    pub mesh_materials: BatchMeshMaterialContainer,
    pub meshes: Vec<MeshResource>,
}

impl Frame {
    pub fn prepare(
        self,
        specialized_mesh_pipeline: &mut SpecializedMeshPipeline,
        pipeline_cache: &mut PipelineCache,
        layouts: &mut MeshVertexBufferLayouts,
        buffer_allocator: &mut BufferAllocator,
        mesh_allocator: &mut MeshAllocator,
        settings: &MeshAllocatorSettings,
        render_device: &RenderDevice,
        render_queue: &RenderQueue,
        mesh_cache: &mut MeshCache,
    ) -> Result<RenderFrame, FrameworkError> {
        mesh_cache.allocate_and_free_meshes(
            settings,
            layouts,
            buffer_allocator,
            render_device,
            render_queue,
            mesh_allocator,
        );

        let mut batchs = vec![];

        for batch_mesh_materials in self.mesh_materials.values() {
            for batch in batch_mesh_materials {
                let pipeline_id = specialized_mesh_pipeline.get(batch, pipeline_cache, layouts)?;
                batchs.push(BatchRenderMeshMaterial {
                    pipeline_id: pipeline_id.id(),
                    mesh: batch.mesh.key(),
                    material: batch.material.clone(),
                });
            }
        }

        Ok(RenderFrame {
            windows: self.windows,
            batchs,
        })
    }
}

pub struct BatchRenderMeshMaterial {
    pub pipeline_id: CachedPipelineId,
    pub mesh: u64,
    pub material: MaterialResource,
}

impl RenderPhase for BatchRenderMeshMaterial {
    fn render(&self, builder: &mut TrackedRenderPassBuilder, context: &RenderPhaseContext) {
        let pipeline = context
            .pipeline_container
            .get_render_pipeline(self.pipeline_id)
            .expect("pipeline must have");
        builder.set_render_pipeline(pipeline);
    }
}

pub struct RenderFrame {
    pub windows: RenderWindows,
    pub batchs: Vec<BatchRenderMeshMaterial>,
}
