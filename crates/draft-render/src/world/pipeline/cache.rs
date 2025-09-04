use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
};

use draft_gfx_base::{PrimitiveState, VertexBufferLayout};

use super::PipelineLayoutCache;

use crate::{
    BindGroupLayout, FragmentState, FrameworkError, PipelineInfo, ShaderCache, VertexState,
};

use draft_gfx_base::{
    BindGroupLayoutDescriptor, CachedPipelineId, DepthStencilState, GetPipelineContainer,
    MultisampleState, Pipeline, PipelineContainer, RawFragmentState, RawPipelineCompilationOptions,
    RawRenderPipelineDescriptor, RawVertexAttribute, RawVertexBufferLayout, RawVertexState,
    RenderDevice, RenderPipeline,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub vertex: VertexState,
    pub fragment: Option<FragmentState>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub layouts: Vec<BindGroupLayoutDescriptor>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ComputePipelineDescriptor {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PipelineDescriptor {
    RenderPipelineDescriptor(Box<RenderPipelineDescriptor>),
    ComputePipelineDescriptor(Box<ComputePipelineDescriptor>),
}

impl PipelineDescriptor {
    pub fn new(
        pipeline_info: &PipelineInfo,
        layouts: &[BindGroupLayoutDescriptor],
        buffers: &[VertexBufferLayout],
    ) -> Self {
        match pipeline_info {
            PipelineInfo::RenderPipelineInfo(render_pipeline_info) => {
                let mut desc: RenderPipelineDescriptor = RenderPipelineDescriptor {
                    label: render_pipeline_info.label.to_string(),
                    vertex: render_pipeline_info.vertex.clone(),
                    fragment: render_pipeline_info.fragment.clone(),
                    primitive: render_pipeline_info.primitive,
                    depth_stencil: render_pipeline_info.depth_stencil.clone(),
                    multisample: render_pipeline_info.multisample,
                    layouts: layouts.to_vec(),
                };

                desc.vertex.buffers.append(&mut buffers.to_vec());

                PipelineDescriptor::RenderPipelineDescriptor(Box::new(desc))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug)]
pub struct CachedPipeline {
    pub descriptor: PipelineDescriptor,
    pub state: PipelineState,
}

#[derive(Debug)]
pub enum PipelineState {
    Queue,
    Ok(Pipeline),
    Error(FrameworkError),
}

impl PipelineState {
    pub fn as_pipeline_ref(&self) -> Option<&Pipeline> {
        match self {
            PipelineState::Ok(pipeline) => Some(pipeline),
            _ => None,
        }
    }
}

pub struct PipelineCache {
    shader_cache: ShaderCache,
    pipeline_layout_cache: PipelineLayoutCache,
    pipelines: Vec<CachedPipeline>,
    new_pipelines: Vec<CachedPipeline>,
    pipeline_map: HashMap<PipelineDescriptor, CachedPipelineId>,
    waiting_pipelines: HashSet<CachedPipelineId>,
    device: RenderDevice,
}

impl PipelineCache {
    pub fn new(device: RenderDevice) -> Self {
        Self {
            shader_cache: Default::default(),
            pipeline_layout_cache: Default::default(),
            pipelines: Default::default(),
            new_pipelines: Default::default(),
            pipeline_map: Default::default(),
            waiting_pipelines: Default::default(),
            device,
        }
    }

    pub fn has_render_pipeline(&self, id: CachedPipelineId) -> bool {
        if let Some(pipeline) = self.pipelines.get(id) {
            match &pipeline.state {
                PipelineState::Ok(pipeline) => match pipeline {
                    Pipeline::RenderPipeline(_) => true,
                    Pipeline::ComputePipeline(_) => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn get_or_create_bind_group_layout(
        &mut self,
        desc: &BindGroupLayoutDescriptor,
    ) -> &BindGroupLayout {
        self.pipeline_layout_cache
            .get_or_create_bind_group_layout(&self.device, desc)
    }

    pub fn get_or_create(&mut self, desc: &PipelineDescriptor) -> CachedPipelineId {
        if self.pipeline_map.contains_key(desc) {
            *self.pipeline_map.get(desc).unwrap()
        } else {
            let id = self.pipelines.len() + self.new_pipelines.len();
            let pipeline = CachedPipeline {
                descriptor: desc.clone(),
                state: PipelineState::Queue,
            };

            self.new_pipelines.push(pipeline);
            self.pipeline_map.insert(desc.clone(), id);

            id
        }
    }

    fn create_render_pipeline(
        &mut self,
        _id: CachedPipelineId,
        desc: RenderPipelineDescriptor,
    ) -> PipelineState {
        match create_pipeline_with_render_pipeline_descriptor(
            &self.device,
            &desc,
            &mut self.shader_cache,
            &mut self.pipeline_layout_cache,
        ) {
            Ok(pipeline) => PipelineState::Ok(pipeline),
            Err(e) => PipelineState::Error(e),
        }
    }

    fn process_pipeline(&mut self, cached_pipeline: &mut CachedPipeline, id: usize) {
        if let PipelineState::Queue = &mut cached_pipeline.state {
            cached_pipeline.state = match &cached_pipeline.descriptor {
                PipelineDescriptor::RenderPipelineDescriptor(descriptor) => {
                    self.create_render_pipeline(id, *descriptor.clone())
                }
                PipelineDescriptor::ComputePipelineDescriptor(_descriptor) => {
                    unimplemented!()
                }
            };
        }
    }

    pub fn process(&mut self) {
        let mut waiting_pipelines = mem::take(&mut self.waiting_pipelines);
        let mut pipelines = mem::take(&mut self.pipelines);

        let mut new_pipelines = mem::take(&mut self.new_pipelines);

        for new_pipeline in new_pipelines.drain(..) {
            let id = pipelines.len();
            pipelines.push(new_pipeline);
            waiting_pipelines.insert(id);
        }

        for id in waiting_pipelines {
            self.process_pipeline(&mut pipelines[id], id);
        }

        self.pipelines = pipelines;
    }
}

impl GetPipelineContainer for PipelineCache {
    fn get_pipeline_container(&self) -> PipelineContainer {
        let pipelines_len = self.pipelines.len();
        let new_pipelines_len = self.new_pipelines.len();
        let len = pipelines_len + new_pipelines_len;

        let mut pipelines = vec![];

        for i in 0..len {
            if i < pipelines_len {
                pipelines.push(self.pipelines[i].state.as_pipeline_ref().cloned());
            } else {
                pipelines.push(None);
            }
        }

        PipelineContainer::new(pipelines)
    }
}

fn create_pipeline_with_render_pipeline_descriptor(
    device: &RenderDevice,
    desc: &RenderPipelineDescriptor,
    shader_cache: &mut ShaderCache,
    pipeline_layout_cache: &mut PipelineLayoutCache,
) -> Result<Pipeline, FrameworkError> {
    let vertex_module = shader_cache.get(device, &desc.vertex.shader)?.clone();
    let fragment_module = match &desc.fragment {
        Some(fragment) => match shader_cache.get(device, &fragment.shader) {
            Ok(module) => Some(module.clone()),
            Err(err) => return Err(err),
        },
        None => None,
    };

    let pipeline_layout =
        pipeline_layout_cache.get_or_create_pipeline_layout(device, &desc.layouts);

    let vertex_buffer_layouts = desc
        .vertex
        .buffers
        .iter()
        .map(|layout| {
            (
                layout.array_stride,
                layout
                    .attributes
                    .iter()
                    .map(|attribute| attribute.into())
                    .collect::<Vec<RawVertexAttribute>>(),
                layout.step_mode,
            )
        })
        .collect::<Vec<_>>();
    let vertex_buffer_layouts = vertex_buffer_layouts
        .iter()
        .map(
            |(array_stride, attributes, step_mode)| RawVertexBufferLayout {
                array_stride: *array_stride,
                attributes,
                step_mode: (*step_mode).into(),
            },
        )
        .collect::<Vec<_>>();

    let fragment_data = desc.fragment.clone().map(|fragment| {
        (
            fragment_module.unwrap(),
            fragment.entry_point,
            fragment
                .targets
                .iter()
                .map(|target| target.as_ref().map(|target| target.into()))
                .collect::<Vec<_>>(),
        )
    });

    let descriptor = RawRenderPipelineDescriptor {
        multiview: None,
        depth_stencil: desc
            .depth_stencil
            .as_ref()
            .map(|depth_stencil| depth_stencil.into()),
        label: Some(&desc.label),
        layout: Some(pipeline_layout.get_pipeline_layout()),
        multisample: desc.multisample.into(),
        primitive: desc.primitive.into(),
        vertex: RawVertexState {
            buffers: &vertex_buffer_layouts,
            entry_point: desc.vertex.entry_point.as_deref(),
            module: &vertex_module,
            compilation_options: RawPipelineCompilationOptions::default(),
        },
        fragment: fragment_data
            .as_ref()
            .map(|(module, entry_point, targets)| RawFragmentState {
                entry_point: entry_point.as_deref(),
                module,
                targets,
                compilation_options: RawPipelineCompilationOptions::default(),
            }),
        cache: None,
    };

    let pipeline = device.wgpu_device().create_render_pipeline(&descriptor);

    Ok(Pipeline::RenderPipeline(RenderPipeline::new(pipeline)))
}
