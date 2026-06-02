use std::{borrow::Cow, collections::HashMap, sync::Arc};

use draft_graphics::{
    BindGroupLayout, ColorTargetState, DepthStencilState, FragmentState, MultisampleState,
    Pipeline, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState,
    RenderDevice, RenderPipelineDescriptor, ShaderModule,
    VertexBufferLayout as RawVertexBufferLayout, VertexState,
};
use draft_mesh::VertexBufferLayout;
use draft_shader::ShaderResource;
use wgpu::ShaderModuleDescriptor;

use crate::{
    FrameworkError,
    frame_graph::{GetPipelineContainer, PipelineContainer},
    render_resource::BindGroupLayoutDescriptor,
};

pub type CachePipelineId = usize;

#[derive(Clone)]
pub struct GpuVertexState {
    pub shader: ShaderResource,
    pub entry_point: Option<Cow<'static, str>>,
    pub buffers: Vec<VertexBufferLayout>,
}

#[derive(Clone)]
pub struct GpuFragmentState {
    pub shader: ShaderResource,
    pub entry_point: Option<Cow<'static, str>>,
    pub targets: Vec<Option<ColorTargetState>>,
}

#[derive(Clone, Default)]
pub struct GpuPipelineLayoutDescriptor {
    bind_group_layouts: HashMap<u32, BindGroupLayoutDescriptor>,
    max_bind_group_index: u32,
}

impl GpuPipelineLayoutDescriptor {
    pub fn create_bind_group_layouts(&self) -> Vec<Option<BindGroupLayoutDescriptor>> {
        let mut bind_group_layouts = vec![];
        for i in 0..=self.max_bind_group_index {
            bind_group_layouts.push(self.bind_group_layouts.get(&i).cloned());
        }

        bind_group_layouts
    }

    pub fn add_bind_group_layout(&mut self, index: u32, layout: BindGroupLayoutDescriptor) {
        self.bind_group_layouts.insert(index, layout);
        self.max_bind_group_index = self.max_bind_group_index.max(index);
    }
}

#[derive(Clone)]
pub struct GpuRenderPipelineDescriptor {
    pub label: String,
    pub layout: GpuPipelineLayoutDescriptor,
    pub vertex: GpuVertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<GpuFragmentState>,
    pub zero_initialize_workgroup_memory: bool,
}

#[derive(Default)]
pub struct BindGroupLayoutCache(HashMap<BindGroupLayoutDescriptor, BindGroupLayout>);

impl BindGroupLayoutCache {
    pub fn get_or_create(
        &mut self,
        render_device: &RenderDevice,
        descriptor: BindGroupLayoutDescriptor,
    ) -> BindGroupLayout {
        self.0
            .entry(descriptor.clone())
            .or_insert_with(|| {
                render_device
                    .create_bind_group_layout(descriptor.label.as_ref(), &descriptor.entries)
            })
            .clone()
    }
}

#[derive(Default)]
pub struct PipelineLayoutCache(HashMap<Vec<Option<BindGroupLayoutDescriptor>>, PipelineLayout>);

impl PipelineLayoutCache {
    pub fn get_or_create(
        &mut self,
        render_device: &RenderDevice,
        descriptors: &[Option<BindGroupLayoutDescriptor>],
        bind_group_layout_cache: &mut BindGroupLayoutCache,
    ) -> PipelineLayout {
        self.0
            .entry(descriptors.to_vec())
            .or_insert_with(|| {
                let mut bind_group_layouts = vec![];

                for descriptor in descriptors.iter() {
                    bind_group_layouts.push(descriptor.as_ref().map(|descriptor| {
                        bind_group_layout_cache.get_or_create(render_device, descriptor.clone())
                    }));
                }

                render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &bind_group_layouts
                        .iter()
                        .map(|layout| layout.as_ref())
                        .collect::<Vec<_>>(),
                    immediate_size: 0,
                })
            })
            .clone()
    }
}

pub struct State {
    pub pipeline: Pipeline,
    pub desc: GpuRenderPipelineDescriptor,
}

pub struct PipelineCache {
    device: RenderDevice,
    shader_cache: HashMap<usize, Arc<ShaderModule>>,
    pipelines: Vec<Option<State>>,
    bind_group_layout_cache: BindGroupLayoutCache,
    pipeline_layout_cache: PipelineLayoutCache,
}

impl GetPipelineContainer for PipelineCache {
    fn get_pipeline_container(&self) -> PipelineContainer {
        let mut pipelines = PipelineContainer::default();

        for state in self.pipelines.iter() {
            pipelines.push(state.as_ref().map(|state| state.pipeline.clone()));
        }

        pipelines
    }
}

impl PipelineCache {
    pub fn new(device: &RenderDevice) -> Self {
        Self {
            device: device.clone(),
            shader_cache: HashMap::default(),
            pipelines: Default::default(),
            bind_group_layout_cache: Default::default(),
            pipeline_layout_cache: Default::default(),
        }
    }

    pub fn create_shader_moudule(&mut self, shader: &ShaderResource) -> Arc<ShaderModule> {
        let shader_data = shader.data_ref();

        let id = shader_data.cache_index.get();

        let shader_module = self.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: shader_data.source.get_shader_source(),
        });

        let shader_module = Arc::new(shader_module);

        self.shader_cache.insert(id, shader_module.clone());

        shader_module
    }

    pub fn create_render_pipeline(
        &mut self,
        desc: &GpuRenderPipelineDescriptor,
    ) -> Result<CachePipelineId, FrameworkError> {
        let buffers = desc
            .vertex
            .buffers
            .iter()
            .map(|buffer| {
                (
                    buffer.array_stride,
                    buffer.step_mode,
                    buffer
                        .attributes
                        .iter()
                        .map(|attribute| attribute.get_wgpu_vertex_attribute())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let buffers = buffers
            .iter()
            .map(|buffer| RawVertexBufferLayout {
                array_stride: buffer.0,
                step_mode: buffer.1,
                attributes: &buffer.2,
            })
            .collect::<Vec<_>>();

        let vertext_state = VertexState {
            module: &self.create_shader_moudule(&desc.vertex.shader),
            entry_point: desc.vertex.entry_point.as_deref(),
            compilation_options: PipelineCompilationOptions {
                constants: &[],
                zero_initialize_workgroup_memory: desc.zero_initialize_workgroup_memory,
            },
            buffers: &buffers,
        };

        let fragment_state = match &desc.fragment {
            None => None,
            Some(frame_state) => Some(FragmentState {
                module: &self.create_shader_moudule(&frame_state.shader),
                entry_point: frame_state.entry_point.as_deref(),
                compilation_options: PipelineCompilationOptions {
                    constants: &[],
                    zero_initialize_workgroup_memory: desc.zero_initialize_workgroup_memory,
                },
                targets: &frame_state.targets,
            }),
        };

        let pipeline_layout = self.pipeline_layout_cache.get_or_create(
            &self.device,
            &desc.layout.create_bind_group_layouts(),
            &mut self.bind_group_layout_cache,
        );

        let render_pipeline = self
            .device
            .create_render_pipelie(&RenderPipelineDescriptor {
                vertex: vertext_state,
                fragment: fragment_state,
                label: Some(&desc.label),
                layout: Some(&pipeline_layout),
                primitive: desc.primitive,
                multisample: desc.multisample,
                depth_stencil: desc.depth_stencil.clone(),
                multiview_mask: None,
                cache: None,
            });

        let len = self.pipelines.len();

        self.pipelines.push(Some(State {
            pipeline: Pipeline::RenderPipeline(render_pipeline),
            desc: desc.clone(),
        }));

        Ok(len)
    }
}
