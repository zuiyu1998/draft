mod layout;

pub use layout::*;

use std::{collections::HashSet, mem, ops::Deref, sync::Arc};

use draft_graphics::{
    ColorTargetState, DepthStencilState, MultisampleState, PipelineCompilationOptions,
    PrimitiveState, PushConstantRange,
    gfx_base::{
        BindGroupLayout, CachedPipelineId, FragmentState as GpuFragmentState, GpuPipeline,
        RenderDevice, RenderPipelineDescriptor as GpuRenderPipelineDescriptor, VertexBufferLayout,
        VertexState as GpuVertexState,
    },
};
use draft_shader::{ShaderCache, ShaderCacheError, ShaderDefVal, ShaderResource};
use fyrox_core::{futures::executor::block_on, parking_lot::Mutex};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct VertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub buffers: Vec<VertexBufferLayout>,
    pub shader_defs: Vec<ShaderDefVal>,
}

#[derive(Debug, Clone)]
pub struct FragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<String>,
    pub targets: Vec<Option<ColorTargetState>>,
    pub shader_defs: Vec<ShaderDefVal>,
}

#[derive(Debug, Clone)]
pub struct RenderPipelineDescriptor {
    pub label: Option<String>,
    pub layout: Vec<BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
    pub vertex: VertexState,
    pub fragment: Option<FragmentState>,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub primitive: PrimitiveState,
    pub zero_initialize_workgroup_memory: bool,
}

#[derive(Debug, Error)]
pub enum PipelineCacheError {
    #[error(transparent)]
    ShaderCacheError(#[from] ShaderCacheError),
}

#[derive(Debug)]
pub enum CachedPipelineState {
    Queued,
    // Creating(Task<Result<Pipeline, PipelineCacheError>>),
    Ok(GpuPipeline),
    Err(PipelineCacheError),
}

pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub state: CachedPipelineState,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedRenderPipelineId(CachedPipelineId);

impl CachedRenderPipelineId {
    pub const INVALID: Self = CachedRenderPipelineId(usize::MAX);

    #[inline]
    pub fn id(&self) -> usize {
        self.0
    }
}

pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
}

fn create_pipeline_task(
    task: impl Future<Output = Result<GpuPipeline, PipelineCacheError>> + Send + 'static,
    _sync: bool,
) -> CachedPipelineState {
    match block_on(task) {
        Ok(pipeline) => CachedPipelineState::Ok(pipeline),
        Err(err) => CachedPipelineState::Err(err),
    }
}

pub struct PipelineCache {
    _device: RenderDevice,
    new_pipelines: Mutex<Vec<CachedPipeline>>,
    pipelines: Vec<CachedPipeline>,
    waiting_pipelines: HashSet<CachedPipelineId>,
    layout_cache: Arc<Mutex<LayoutCache>>,
    shader_cache: Arc<Mutex<ShaderCache>>,
}

impl PipelineCache {
    pub fn new(device: RenderDevice) -> Self {
        Self {
            _device: device,
            new_pipelines: Default::default(),
            pipelines: Default::default(),
            waiting_pipelines: Default::default(),
            layout_cache: Arc::new(Mutex::new(LayoutCache::default())),
            shader_cache: Arc::new(Mutex::new(ShaderCache::new())),
        }
    }

    pub fn process_queue(&mut self) {
        let mut waiting_pipelines = mem::take(&mut self.waiting_pipelines);
        let mut pipelines = mem::take(&mut self.pipelines);

        {
            let mut new_pipelines = self.new_pipelines.lock();
            for new_pipeline in new_pipelines.drain(..) {
                let id = pipelines.len();
                pipelines.push(new_pipeline);
                waiting_pipelines.insert(id);
            }
        }

        for id in waiting_pipelines {
            self.process_pipeline(&mut pipelines[id], id);
        }

        self.pipelines = pipelines;
    }

    fn process_pipeline(&mut self, cached_pipeline: &mut CachedPipeline, id: usize) {
        match &mut cached_pipeline.state {
            CachedPipelineState::Queued => {
                cached_pipeline.state = match &cached_pipeline.descriptor {
                    PipelineDescriptor::RenderPipelineDescriptor(descriptor) => {
                        self.start_create_render_pipeline(id, *descriptor.clone())
                    }
                };
            }

            CachedPipelineState::Ok(_) => return,
            _ => return,
        }

        self.waiting_pipelines.insert(id);
    }

    fn start_create_render_pipeline(
        &mut self,
        id: CachedPipelineId,
        descriptor: RenderPipelineDescriptor,
    ) -> CachedPipelineState {
        let device = self._device.clone();

        let shader_cache = self.shader_cache.clone();
        let layout_cache = self.layout_cache.clone();

        create_pipeline_task(
            async move {
                let mut shader_cache = shader_cache.lock();
                let mut layout_cache = layout_cache.lock();

                let vertex_module = match shader_cache.get(
                    &device,
                    id,
                    &descriptor.vertex.shader,
                    &descriptor.vertex.shader_defs,
                ) {
                    Ok(module) => module,
                    Err(err) => return Err(err.into()),
                };

                let fragment_module = match &descriptor.fragment {
                    Some(fragment) => {
                        match shader_cache.get(&device, id, &fragment.shader, &fragment.shader_defs)
                        {
                            Ok(module) => Some(module),
                            Err(err) => return Err(err.into()),
                        }
                    }
                    None => None,
                };

                let layout =
                    if descriptor.layout.is_empty() && descriptor.push_constant_ranges.is_empty() {
                        None
                    } else {
                        Some(layout_cache.get(
                            &device,
                            &descriptor.layout,
                            descriptor.push_constant_ranges.to_vec(),
                        ))
                    };

                drop((shader_cache, layout_cache));

                let fragment_data = descriptor.fragment.as_ref().map(|fragment| {
                    (
                        fragment_module.unwrap(),
                        fragment.entry_point.clone(),
                        fragment.targets.as_slice(),
                    )
                });

                // TODO: Expose the rest of this somehow
                let compilation_options = PipelineCompilationOptions {
                    constants: &[],
                    zero_initialize_workgroup_memory: descriptor.zero_initialize_workgroup_memory,
                };

                let descriptor = GpuRenderPipelineDescriptor {
                    label: descriptor.label,
                    depth_stencil: descriptor.depth_stencil.clone(),
                    layout: layout.as_ref().map(|layout| layout.deref().clone()),
                    multisample: descriptor.multisample,
                    primitive: descriptor.primitive,
                    vertex: GpuVertexState {
                        buffers: descriptor.vertex.buffers.clone(),
                        entry_point: descriptor.vertex.entry_point.clone(),
                        module: vertex_module.value().clone(),
                        // TODO: Should this be the same as the fragment compilation options?
                        compilation_options: compilation_options.clone(),
                    },
                    fragment: fragment_data
                        .as_ref()
                        .map(|(module, entry_point, targets)| GpuFragmentState {
                            entry_point: entry_point.clone(),
                            module: module.value().clone(),
                            targets: targets.to_vec(),
                            // TODO: Should this be the same as the vertex compilation options?
                            compilation_options,
                        }),
                };

                Ok(GpuPipeline::RenderPipeline(
                    device.create_render_pipeline(descriptor),
                ))
            },
            false,
        )
    }

    pub fn queue_render_pipeline(
        &self,
        descriptor: RenderPipelineDescriptor,
    ) -> CachedRenderPipelineId {
        let mut new_pipelines = self.new_pipelines.lock();
        let id = CachedRenderPipelineId(self.pipelines.len() + new_pipelines.len());
        new_pipelines.push(CachedPipeline {
            descriptor: PipelineDescriptor::RenderPipelineDescriptor(Box::new(descriptor)),
            state: CachedPipelineState::Queued,
        });
        id
    }
}
