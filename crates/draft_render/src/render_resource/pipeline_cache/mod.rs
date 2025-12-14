use std::{collections::HashSet, mem};

use draft_graphics::frame_graph::gfx_base::{CachedPipelineId, GpuPipeline, RenderDevice};
use fyrox_core::parking_lot::Mutex;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct RenderPipelineDescriptor {}

#[derive(Debug, Error)]
pub enum PipelineCacheError {}

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

pub struct PipelineCache {
    _device: RenderDevice,
    new_pipelines: Mutex<Vec<CachedPipeline>>,
    pipelines: Vec<CachedPipeline>,
    waiting_pipelines: HashSet<CachedPipelineId>,
}

impl PipelineCache {
    pub fn new(device: RenderDevice) -> Self {
        Self {
            _device: device,
            new_pipelines: Default::default(),
            pipelines: Default::default(),
            waiting_pipelines: Default::default(),
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
        _id: CachedPipelineId,
        _descriptor: RenderPipelineDescriptor,
    ) -> CachedPipelineState {
        let _device = self._device.clone();
        todo!()

        // let shader_cache = self.shader_cache.clone();
        // let layout_cache = self.layout_cache.clone();

        // create_pipeline_task(
        //     async move {
        //         let mut shader_cache = shader_cache.lock().unwrap();
        //         let mut layout_cache = layout_cache.lock().unwrap();

        //         let vertex_module = match shader_cache.get(
        //             &device,
        //             id,
        //             descriptor.vertex.shader.id(),
        //             &descriptor.vertex.shader_defs,
        //         ) {
        //             Ok(module) => module,
        //             Err(err) => return Err(err),
        //         };

        //         let fragment_module = match &descriptor.fragment {
        //             Some(fragment) => {
        //                 match shader_cache.get(
        //                     &device,
        //                     id,
        //                     fragment.shader.id(),
        //                     &fragment.shader_defs,
        //                 ) {
        //                     Ok(module) => Some(module),
        //                     Err(err) => return Err(err),
        //                 }
        //             }
        //             None => None,
        //         };

        //         let layout =
        //             if descriptor.layout.is_empty() && descriptor.push_constant_ranges.is_empty() {
        //                 None
        //             } else {
        //                 Some(layout_cache.get(
        //                     &device,
        //                     &descriptor.layout,
        //                     descriptor.push_constant_ranges.to_vec(),
        //                 ))
        //             };

        //         drop((shader_cache, layout_cache));

        //         let vertex_buffer_layouts = descriptor
        //             .vertex
        //             .buffers
        //             .iter()
        //             .map(|layout| RawVertexBufferLayout {
        //                 array_stride: layout.array_stride,
        //                 attributes: &layout.attributes,
        //                 step_mode: layout.step_mode,
        //             })
        //             .collect::<Vec<_>>();

        //         let fragment_data = descriptor.fragment.as_ref().map(|fragment| {
        //             (
        //                 fragment_module.unwrap(),
        //                 fragment.entry_point.as_deref(),
        //                 fragment.targets.as_slice(),
        //             )
        //         });

        //         // TODO: Expose the rest of this somehow
        //         let compilation_options = PipelineCompilationOptions {
        //             constants: &[],
        //             zero_initialize_workgroup_memory: descriptor.zero_initialize_workgroup_memory,
        //         };

        //         let descriptor = RawRenderPipelineDescriptor {
        //             multiview: None,
        //             depth_stencil: descriptor.depth_stencil.clone(),
        //             label: descriptor.label.as_deref(),
        //             layout: layout.as_ref().map(|layout| -> &PipelineLayout { layout }),
        //             multisample: descriptor.multisample,
        //             primitive: descriptor.primitive,
        //             vertex: RawVertexState {
        //                 buffers: &vertex_buffer_layouts,
        //                 entry_point: descriptor.vertex.entry_point.as_deref(),
        //                 module: &vertex_module,
        //                 // TODO: Should this be the same as the fragment compilation options?
        //                 compilation_options: compilation_options.clone(),
        //             },
        //             fragment: fragment_data
        //                 .as_ref()
        //                 .map(|(module, entry_point, targets)| RawFragmentState {
        //                     entry_point: entry_point.as_deref(),
        //                     module,
        //                     targets,
        //                     // TODO: Should this be the same as the vertex compilation options?
        //                     compilation_options,
        //                 }),
        //             cache: None,
        //         };

        //         Ok(GpuPipeline::RenderPipeline(
        //             device.create_render_pipeline(&descriptor),
        //         ))
        //     },
        //     self.synchronous_pipeline_compilation,
        // )
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
